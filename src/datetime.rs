#[cfg(feature = "chrono")]
use chrono::{DateTime as ChronoDateTime, Datelike, TimeZone, Timelike, Utc};
#[cfg(feature = "tz")]
use chrono_tz::Tz;

#[cfg(not(feature = "chrono"))]
use std::time::{SystemTime, UNIX_EPOCH};

// Static mapping of a few common zones to their fixed offsets in seconds (no DST).
#[cfg(not(feature = "tz"))]
const STATIC_ZONES: &[(&str, i32)] = &[
    ("UTC", 0),
    ("America/New_York", -5 * 3600),
    ("America/Los_Angeles", -8 * 3600),
    ("Europe/London", 0),
    ("Europe/Paris", 1 * 3600),
    ("Asia/Tokyo", 9 * 3600),
    ("Asia/Shanghai", 8 * 3600),
    ("Australia/Sydney", 10 * 3600),
    ("Asia/Kolkata", 5 * 3600 + 30 * 60),
    ("America/Sao_Paulo", -3 * 3600),
];

use crate::duration::Duration;
#[cfg(feature = "chrono")]
use crate::format::format_datetime;
use crate::locale;

/// A date and time value with timezone support.
///
/// `DateTime` is immutable – all operations return new instances.
/// In zero-deps mode, times are always UTC. Enable the `tz` feature for timezone support.
///
/// # Examples
///
/// ```rust
/// use tempotime::{DateTime, Duration};
///
/// // Get current time
/// let now = DateTime::now();
///
/// // Parse ISO string
/// let dt = DateTime::from_iso("2025-10-30T14:30:00Z").unwrap();
///
/// // Add duration
/// let future = dt.plus(&Duration::from_object(&[("days", 7)]));
///
/// // Format output
/// println!("{}", future.to_format("yyyy-MM-dd HH:mm:ss"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    #[cfg(feature = "chrono")]
    inner: ChronoDateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    timestamp_ms: i64,
    #[cfg(feature = "tz")]
    zone: Option<Tz>,
    // In zero-deps builds we support a small static zone map via set_zone()
    #[cfg(not(feature = "tz"))]
    _zone_applied: bool,
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        #[cfg(feature = "chrono")]
        return self.inner.partial_cmp(&other.inner);
        #[cfg(not(feature = "chrono"))]
        return self.timestamp_ms.partial_cmp(&other.timestamp_ms);
    }
}

impl DateTime {
    /// Creates a DateTime representing the current moment in UTC.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tempotime::DateTime;
    ///
    /// let now = DateTime::now();
    /// println!("Current time: {}", now.to_iso());
    /// ```
    #[cfg(feature = "chrono")]
    pub fn now() -> Self {
        DateTime {
            inner: Utc::now(),
            #[cfg(feature = "tz")]
            zone: None,
            #[cfg(not(feature = "tz"))]
            _zone_applied: false,
        }
    }

    #[cfg(not(feature = "chrono"))]
    pub fn now() -> Self {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        DateTime {
            timestamp_ms: duration.as_millis() as i64,
            #[cfg(not(feature = "tz"))]
            _zone_applied: false,
        }
    }

    #[cfg(feature = "chrono")]
    pub fn local() -> Self {
        let local = chrono::Local::now();
        DateTime {
            inner: local.with_timezone(&Utc),
            #[cfg(feature = "tz")]
            zone: None,
            #[cfg(not(feature = "tz"))]
            _zone_applied: false,
        }
    }

    #[cfg(not(feature = "chrono"))]
    pub fn local() -> Self {
        Self::now()
    }

    #[cfg(feature = "chrono")]
    pub fn from_iso(s: &str) -> Result<Self, String> {
        s.parse::<ChronoDateTime<Utc>>()
            .map(|dt| DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: None,
                #[cfg(not(feature = "tz"))]
                _zone_applied: false,
            })
            .map_err(|e| format!("Invalid ISO date: {}", e))
    }

    #[cfg(not(feature = "chrono"))]
    pub fn from_iso(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.len() < 19 {
            return Err("ISO string too short".to_string());
        }

        let year: i32 = s[0..4].parse().map_err(|_| "Invalid year")?;
        let month: u32 = s[5..7].parse().map_err(|_| "Invalid month")?;
        let day: u32 = s[8..10].parse().map_err(|_| "Invalid day")?;
        let hour: u32 = s[11..13].parse().map_err(|_| "Invalid hour")?;
        let minute: u32 = s[14..16].parse().map_err(|_| "Invalid minute")?;
        let second: u32 = s[17..19].parse().map_err(|_| "Invalid second")?;

        let timestamp_ms = Self::compute_timestamp(year, month, day, hour, minute, second, 0);
    Ok(DateTime { timestamp_ms, #[cfg(not(feature = "tz"))] _zone_applied: false })
    }

    pub fn from_format(s: &str, fmt: &str) -> Result<Self, String> {
        // Simple parser for patterns similar to to_format tokens.
        // Supported tokens: yyyy, yy, MMMM, MMM, MM, M, dd, d, do, H/H H, HH, h/h hh, m/mm, s/ss, SSS, a
        let input = s;
        let mut ix: usize = 0;
        let mut year: Option<i32> = None;
        let mut month: Option<u32> = None;
        let mut day: Option<u32> = None;
        let mut hour: Option<u32> = None;
        let mut minute: Option<u32> = None;
        let mut second: Option<u32> = None;
        let mut millis: Option<u32> = None;
        let mut pm = false;

        let mut chars = fmt.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '\'' => {
                    // literal until next '\''; support escape of single-quote via doubled '' per common patterns
                    let mut lit = String::new();
                    loop {
                        match chars.next() {
                            Some(c2) => {
                                if c2 == '\'' {
                                    // if next char is also a single-quote, that's an escaped quote -> append one and continue
                                    if chars.peek() == Some(&'\'') {
                                        // consume the escaped quote and append a single quote
                                        chars.next();
                                        lit.push('\'');
                                        continue;
                                    }
                                    // otherwise it's the closing quote
                                    break;
                                }
                                lit.push(c2);
                            }
                            None => {
                                return Err("Unterminated literal in format string".to_string());
                            }
                        }
                    }
                    // match literal in input at current position
                    if input.get(ix..).map_or(false, |s| s.starts_with(&lit)) {
                        ix += lit.len();
                    } else {
                        return Err(format!("Literal '{}' not found at input position {}", lit, ix));
                    }
                }
                'y' => {
                    let count = 1 + chars.clone().take_while(|&c| c == 'y').count();
                    for _ in 1..count { chars.next(); }
                    if count >= 4 {
                        if ix + 4 > input.len() { return Err("Unexpected end while parsing year".to_string()); }
                        let v: i32 = input[ix..ix+4].parse().map_err(|_| "Invalid year")?;
                        year = Some(v);
                        ix += 4;
                    } else {
                        if ix + 2 > input.len() { return Err("Unexpected end while parsing year".to_string()); }
                        let v: i32 = input[ix..ix+2].parse().map_err(|_| "Invalid year")?;
                        // two-digit year: assume 2000-2099 for simplicity
                        year = Some(2000 + v);
                        ix += 2;
                    }
                }
                'M' => {
                    let count = 1 + chars.clone().take_while(|&c| c == 'M').count();
                    for _ in 1..count { chars.next(); }
                    if count >= 4 {
                        // full month name - try matching any month name (case-insensitive)
                        let names = ["January","February","March","April","May","June","July","August","September","October","November","December"];
                        let mut matched = None;
                        for (i,name) in names.iter().enumerate() {
                            let nl = name.len();
                            if input.len() >= ix + nl && input[ix..ix+nl].eq_ignore_ascii_case(name) {
                                matched = Some((i+1) as u32);
                                ix += nl;
                                break;
                            }
                        }
                        if matched.is_none() { return Err("Month name not found".to_string()); }
                        month = matched;
                    } else if count == 3 {
                        let names = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
                        let mut matched = None;
                        for (i,name) in names.iter().enumerate() {
                            let nl = name.len();
                            if input.len() >= ix + nl && input[ix..ix+nl].eq_ignore_ascii_case(name) {
                                matched = Some((i+1) as u32);
                                ix += nl;
                                break;
                            }
                        }
                        if matched.is_none() { return Err("Short month name not found".to_string()); }
                        month = matched;
                    } else {
                        // numeric month
                        let _digits = if count == 2 {2} else {1};
                        let mut parsed = None;
                        // try 2-digit first if possible
                        if count == 2 && ix + 2 <= input.len() {
                            if let Ok(v) = input[ix..ix+2].parse::<u32>() { parsed = Some((v,2)); }
                        }
                        if parsed.is_none() {
                            // try 1-digit
                            if ix + 1 <= input.len() {
                                if let Ok(v) = input[ix..ix+1].parse::<u32>() { parsed = Some((v,1)); }
                            }
                        }
                        if let Some((v,len)) = parsed { month = Some(v); ix += len; } else { return Err("Invalid month number".to_string()); }
                    }
                }
                'd' => {
                    if chars.peek() == Some(&'o') {
                        chars.next();
                        // ordinal: digits followed by st/nd/rd/th
                        let mut j = ix;
                        while j < input.len() && input.as_bytes()[j].is_ascii_digit() { j += 1; }
                        if j==ix { return Err("Expected day number".to_string()); }
                        let v: u32 = input[ix..j].parse().map_err(|_| "Invalid day")?;
                        // skip suffix letters
                        let mut k = j;
                        while k < input.len() && input.as_bytes()[k].is_ascii_alphabetic() { k += 1; }
                        ix = k;
                        day = Some(v);
                    } else {
                        let count = 1 + chars.clone().take_while(|&c| c == 'd').count();
                        for _ in 1..count { chars.next(); }
                        let len = if count>=2 {2} else {1};
                        if ix + len > input.len() { return Err("Unexpected end while parsing day".to_string()); }
                        let v: u32 = input[ix..ix+len].parse().map_err(|_| "Invalid day")?;
                        day = Some(v);
                        ix += len;
                    }
                }
                'H' | 'h' => {
                    let is_h = ch == 'h';
                    let count = 1 + chars.clone().take_while(|&c| c == ch).count();
                    for _ in 1..count { chars.next(); }
                    let len = if count>=2 {2} else {1};
                    if ix + len > input.len() { return Err("Unexpected end while parsing hour".to_string()); }
                    let v: u32 = input[ix..ix+len].parse().map_err(|_| "Invalid hour")?;
                    hour = Some(v);
                    ix += len;
                    if is_h {
                        // will adjust based on am/pm
                    }
                }
                'm' => {
                    let count = 1 + chars.clone().take_while(|&c| c == 'm').count();
                    for _ in 1..count { chars.next(); }
                    let len = if count>=2 {2} else {1};
                    if ix + len > input.len() { return Err("Unexpected end while parsing minute".to_string()); }
                    let v: u32 = input[ix..ix+len].parse().map_err(|_| "Invalid minute")?;
                    minute = Some(v);
                    ix += len;
                }
                's' => {
                    let count = 1 + chars.clone().take_while(|&c| c == 's').count();
                    for _ in 1..count { chars.next(); }
                    let len = if count>=2 {2} else {1};
                    if ix + len > input.len() { return Err("Unexpected end while parsing second".to_string()); }
                    let v: u32 = input[ix..ix+len].parse().map_err(|_| "Invalid second")?;
                    second = Some(v);
                    ix += len;
                }
                'S' => {
                    let count = 1 + chars.clone().take_while(|&c| c == 'S').count();
                    for _ in 1..count { chars.next(); }
                    // parse milliseconds (up to 3 digits)
                    let mut j = ix;
                    while j < input.len() && input.as_bytes()[j].is_ascii_digit() { j += 1; }
                    if j==ix { return Err("Expected millis".to_string()); }
                    let txt = &input[ix..j];
                    let mut v: u32 = txt.parse().map_err(|_| "Invalid millis")?;
                    // normalize to milliseconds length
                    if txt.len() == 1 { v *= 100; } else if txt.len() == 2 { v *= 10; }
                    millis = Some(v);
                    ix = j;
                }
                'a' => {
                    // am/pm
                    if input[ix..].to_lowercase().starts_with("am") { pm = false; ix += 2; }
                    else if input[ix..].to_lowercase().starts_with("pm") { pm = true; ix += 2; }
                    else { return Err("Expected am or pm".to_string()); }
                }
                other => {
                    // expect literal char
                    let c = other;
                    if ix >= input.len() || input.as_bytes()[ix] as char != c { return Err(format!("Expected '{}'", c)); }
                    ix += 1;
                }
            }
        }

        // fill defaults
        let y = year.unwrap_or(1970);
        let m = month.unwrap_or(1);
        let d = day.unwrap_or(1);
        let mut h = hour.unwrap_or(0);
        let min = minute.unwrap_or(0);
        let sec = second.unwrap_or(0);
        let ms = millis.unwrap_or(0);
        if let Some(_) = hour {
            // if 12-hour clock and pm flag
            if pm {
                if h < 12 { h += 12; }
            } else {
                if h == 12 && fmt.contains('h') { h = 0; }
            }
        }

        #[cfg(feature = "chrono")]
        {
            use chrono::Utc;
            let naive = Utc.with_ymd_and_hms(y, m, d, h, min, sec).single().ok_or("Invalid date")?;
            let dt = naive + chrono::Duration::milliseconds(ms as i64);
            return Ok(DateTime { inner: dt, #[cfg(feature = "tz")] zone: None, #[cfg(not(feature = "tz"))] _zone_applied: false });
        }

        #[cfg(not(feature = "chrono"))]
        {
            let ts = Self::compute_timestamp(y, m, d, h, min, sec, ms);
            return Ok(DateTime { timestamp_ms: ts, #[cfg(feature = "tz")] zone: None, #[cfg(not(feature = "tz"))] _zone_applied: false });
        }
    }

    #[cfg(feature = "tz")]
    pub fn set_zone(mut self, zone: &str) -> Self {
        if let Ok(tz) = zone.parse::<Tz>() {
            self.zone = Some(tz);
        }
        self
    }

    #[cfg(not(feature = "tz"))]
    pub fn set_zone(self, _zone: &str) -> Self {
        // Try to apply a static offset if the zone appears in our STATIC_ZONES map.
        let mut out = self;
        #[cfg(not(feature = "chrono"))]
        {
            let zone_name = _zone;
            if let Some((_, offset)) = STATIC_ZONES.iter().find(|(n, _)| n.eq_ignore_ascii_case(zone_name)) {
                // offset is seconds east of UTC; applying offset shows local wall time
                out.timestamp_ms = out.timestamp_ms + (*offset as i64) * 1000;
                out._zone_applied = true;
            }
        }
        out
    }

    pub fn plus(self, dur: &Duration) -> Self {
        let (years, months, weeks, days, hours, minutes, seconds, millis) = dur.components();
        #[cfg(feature = "chrono")]
        {
            let mut dt = self.inner;

            if years != 0 {
                let new_year = dt.year() + years as i32;
                dt = Utc
                    .with_ymd_and_hms(
                        new_year,
                        dt.month(),
                        dt.day(),
                        dt.hour(),
                        dt.minute(),
                        dt.second(),
                    )
                    .single()
                    .unwrap_or(dt);
            }
            if months != 0 {
                let total_months = dt.month() as i32 + months as i32;
                let new_month = ((total_months - 1).rem_euclid(12) + 1) as u32;
                let year_offset = (total_months - 1).div_euclid(12);
                let new_year = dt.year() + year_offset;
                dt = Utc
                    .with_ymd_and_hms(
                        new_year,
                        new_month,
                        dt.day(),
                        dt.hour(),
                        dt.minute(),
                        dt.second(),
                    )
                    .single()
                    .unwrap_or(dt);
            }

            let total_secs =
                weeks * 7 * 86400 + days * 86400 + hours * 3600 + minutes * 60 + seconds;
            dt += chrono::Duration::seconds(total_secs);
            dt += chrono::Duration::milliseconds(millis);

            DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: self.zone,
                #[cfg(not(feature = "tz"))]
                _zone_applied: false,
            }
        }

        #[cfg(not(feature = "chrono"))]
        {
            // Accurate month/year handling without chrono.
            // 1) Decompose current timestamp into components
            let (mut y, mut m, mut d, h, mi, s, ms) =
                crate::format::decompose_timestamp_ms(self.timestamp_ms);

            // Apply years as months offset
            let total_months: i64 = months as i64 + years as i64 * 12;
            if total_months != 0 {
                let (ny, nm, nd) = add_months_to_ymd(y, m, d, total_months);
                y = ny;
                m = nm;
                d = nd;
            }

            // Recompute base timestamp after year/month/day adjustments
            let base_ts = Self::compute_timestamp(y, m, d, h, mi, s, ms);

            // Apply weeks/days/hours/minutes/seconds/millis as ms offset
            let small_ms = (weeks * 7 * 86400 * 1000)
                + (days * 86400 * 1000)
                + (hours * 3600 * 1000)
                + (minutes * 60 * 1000)
                + (seconds * 1000)
                + millis;

            DateTime {
                timestamp_ms: base_ts + small_ms,
                #[cfg(feature = "tz")]
                zone: self.zone,
                #[cfg(not(feature = "tz"))]
                _zone_applied: false,
            }
        }
    }

    pub fn minus(self, dur: &Duration) -> Self {
        let (years, months, weeks, days, hours, minutes, seconds, millis) = dur.components();
        let negated = Duration::from_object(&[
            ("years", -years),
            ("months", -months),
            ("weeks", -weeks),
            ("days", -days),
            ("hours", -hours),
            ("minutes", -minutes),
            ("seconds", -seconds),
            ("milliseconds", -millis),
        ]);
        self.plus(&negated)
    }

    pub fn start_of(self, unit: &str) -> Self {
        #[cfg(feature = "chrono")]
        {
            let dt = match unit {
                "year" => Utc
                    .with_ymd_and_hms(self.inner.year(), 1, 1, 0, 0, 0)
                    .single()
                    .unwrap(),
                "month" => Utc
                    .with_ymd_and_hms(self.inner.year(), self.inner.month(), 1, 0, 0, 0)
                    .single()
                    .unwrap(),
                "day" => Utc
                    .with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        0,
                        0,
                        0,
                    )
                    .single()
                    .unwrap(),
                "hour" => Utc
                    .with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        self.inner.hour(),
                        0,
                        0,
                    )
                    .single()
                    .unwrap(),
                "minute" => Utc
                    .with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        self.inner.hour(),
                        self.inner.minute(),
                        0,
                    )
                    .single()
                    .unwrap(),
                "second" => self.inner,
                _ => self.inner,
            };
            DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: self.zone,
                #[cfg(not(feature = "tz"))]
                _zone_applied: false,
            }
        }

        #[cfg(not(feature = "chrono"))]
        {
            let (y, m, d, h, mi, s, ms) = crate::format::decompose_timestamp_ms(self.timestamp_ms);
            let (ny, nm, nd, nh, nmi, ns, nms) = match unit {
                "year" => (y, 1, 1, 0, 0, 0, 0),
                "month" => (y, m, 1, 0, 0, 0, 0),
                "day" => (y, m, d, 0, 0, 0, 0),
                "hour" => (y, m, d, h, 0, 0, 0),
                "minute" => (y, m, d, h, mi, 0, 0),
                "second" => (y, m, d, h, mi, s, 0),
                _ => (y, m, d, h, mi, s, ms),
            };
            DateTime {
                timestamp_ms: Self::compute_timestamp(ny, nm, nd, nh, nmi, ns, nms),
                #[cfg(not(feature = "tz"))]
                _zone_applied: false,
            }
        }
    }

    pub fn end_of(self, unit: &str) -> Self {
        #[cfg(feature = "chrono")]
        {
            let dt = match unit {
                "year" => {
                    Utc.with_ymd_and_hms(self.inner.year(), 12, 31, 23, 59, 59)
                        .single()
                        .unwrap()
                        + chrono::Duration::milliseconds(999)
                }
                "month" => {
                    let next_month = if self.inner.month() == 12 {
                        Utc.with_ymd_and_hms(self.inner.year() + 1, 1, 1, 0, 0, 0)
                            .single()
                            .unwrap()
                    } else {
                        Utc.with_ymd_and_hms(self.inner.year(), self.inner.month() + 1, 1, 0, 0, 0)
                            .single()
                            .unwrap()
                    };
                    next_month - chrono::Duration::milliseconds(1)
                }
                "day" => {
                    Utc.with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        23,
                        59,
                        59,
                    )
                    .single()
                    .unwrap()
                        + chrono::Duration::milliseconds(999)
                }
                "hour" => {
                    Utc.with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        self.inner.hour(),
                        59,
                        59,
                    )
                    .single()
                    .unwrap()
                        + chrono::Duration::milliseconds(999)
                }
                "minute" => {
                    Utc.with_ymd_and_hms(
                        self.inner.year(),
                        self.inner.month(),
                        self.inner.day(),
                        self.inner.hour(),
                        self.inner.minute(),
                        59,
                    )
                    .single()
                    .unwrap()
                        + chrono::Duration::milliseconds(999)
                }
                _ => self.inner,
            };
            DateTime {
                inner: dt,
                #[cfg(feature = "tz")]
                zone: self.zone,
            }
        }

        #[cfg(not(feature = "chrono"))]
        {
            let (y, m, d, h, mi, s, _) = crate::format::decompose_timestamp_ms(self.timestamp_ms);
            let (ny, nm, nd, nh, nmi, ns, nms) = match unit {
                "year" => (y, 12, 31, 23, 59, 59, 999),
                "month" => {
                    // compute last day of month
                    let next = if m == 12 {
                        (y + 1, 1, 1)
                    } else {
                        (y, m + 1, 1)
                    };
                    let last_day_ts =
                        Self::compute_timestamp(next.0, next.1, next.2, 0, 0, 0, 0) - 1;
                    let (yy, mm, dd, hh, mn, ss, ms) =
                        crate::format::decompose_timestamp_ms(last_day_ts);
                    (yy, mm, dd, hh, mn, ss, ms)
                }
                "day" => (y, m, d, 23, 59, 59, 999),
                "hour" => (y, m, d, h, 59, 59, 999),
                "minute" => (y, m, d, h, mi, 59, 999),
                _ => (y, m, d, h, mi, s, 0),
            };
            DateTime {
                timestamp_ms: Self::compute_timestamp(ny, nm, nd, nh, nmi, ns, nms),
                #[cfg(not(feature = "tz"))]
                _zone_applied: false,
            }
        }
    }

    pub fn to_iso(&self) -> String {
        #[cfg(feature = "chrono")]
        {
            #[cfg(feature = "tz")]
            if let Some(tz) = self.zone {
                return self.inner.with_timezone(&tz).to_rfc3339();
            }
            self.inner.to_rfc3339()
        }

        #[cfg(not(feature = "chrono"))]
        {
            let (y, m, d, h, mi, s, _) = crate::format::decompose_timestamp_ms(self.timestamp_ms);
            format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", y, m, d, h, mi, s)
        }
    }

    pub fn to_format(&self, fmt: &str) -> String {
        #[cfg(feature = "chrono")]
        {
            #[cfg(feature = "tz")]
            if let Some(tz) = self.zone {
                let _local_dt = self.inner.with_timezone(&tz);
                return format_datetime(&self.inner, fmt);
            }
            format_datetime(&self.inner, fmt)
        }

        #[cfg(not(feature = "chrono"))]
        {
            crate::format::format_datetime_from_ts(self.timestamp_ms, fmt)
        }
    }

    /// Write formatted output directly into the provided writer (zero-allocation except the writer's buffer).
    pub fn format_into<W: core::fmt::Write>(&self, w: &mut W, fmt: &str) -> core::fmt::Result {
        #[cfg(feature = "chrono")]
        {
            #[cfg(feature = "tz")]
            if let Some(tz) = self.zone {
                let _local_dt = self.inner.with_timezone(&tz);
                return crate::format::format_datetime_into(w, &self.inner, fmt);
            }
            return crate::format::format_datetime_into(w, &self.inner, fmt);
        }

        #[cfg(not(feature = "chrono"))]
        {
            return crate::format::format_datetime_from_ts_into(w, self.timestamp_ms, fmt);
        }
    }

    pub fn to_locale_string(&self, preset: &str) -> String {
        #[cfg(feature = "chrono")]
        {
            locale::to_locale_string(&self.inner, preset)
        }
        #[cfg(not(feature = "chrono"))]
        {
            locale::to_locale_string_from_ts(self.timestamp_ms, preset)
        }
    }

    pub fn diff(&self, other: &DateTime, unit: &str) -> f64 {
        #[cfg(feature = "chrono")]
        let diff_ms = (self.inner.timestamp_millis() - other.inner.timestamp_millis()) as f64;
        #[cfg(not(feature = "chrono"))]
        let diff_ms = (self.timestamp_ms - other.timestamp_ms) as f64;
        match unit {
            "milliseconds" | "millisecond" => diff_ms,
            "seconds" | "second" => diff_ms / 1000.0,
            "minutes" | "minute" => diff_ms / (1000.0 * 60.0),
            "hours" | "hour" => diff_ms / (1000.0 * 60.0 * 60.0),
            "days" | "day" => diff_ms / (1000.0 * 60.0 * 60.0 * 24.0),
            "weeks" | "week" => diff_ms / (1000.0 * 60.0 * 60.0 * 24.0 * 7.0),
            "months" | "month" => diff_ms / (1000.0 * 60.0 * 60.0 * 24.0 * 30.0),
            "years" | "year" => diff_ms / (1000.0 * 60.0 * 60.0 * 24.0 * 365.0),
            _ => 0.0,
        }
    }

    #[cfg(not(feature = "chrono"))]
    fn compute_timestamp(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        millis: u32,
    ) -> i64 {
        let days = days_from_civil(year, month, day);
        let secs = days * 86400 + hour as i64 * 3600 + minute as i64 * 60 + second as i64;
        secs * 1000 + millis as i64
    }

    pub const DATE_SHORT: &'static str = locale::DATE_SHORT;
    pub const DATE_MED: &'static str = locale::DATE_MED;
    pub const DATE_FULL: &'static str = locale::DATE_FULL;
    pub const TIME_SIMPLE: &'static str = locale::TIME_SIMPLE;
    pub const TIME_WITH_SECONDS: &'static str = locale::TIME_WITH_SECONDS;
    pub const DATETIME_SHORT: &'static str = locale::DATETIME_SHORT;
    pub const DATETIME_MED: &'static str = locale::DATETIME_MED;
    pub const DATETIME_FULL: &'static str = locale::DATETIME_FULL;
}

#[cfg(not(feature = "chrono"))]
fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = year as i64 - if month <= 2 { 1 } else { 0 };
    let m = month as i64;
    let d = day as i64;
    let era = if y >= 0 { y / 400 } else { (y - 399) / 400 };
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

#[cfg(not(feature = "chrono"))]
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(not(feature = "chrono"))]
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => if is_leap_year(year) { 29 } else { 28 },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    }
}

#[cfg(not(feature = "chrono"))]
fn add_months_to_ymd(year: i32, month: u32, day: u32, offset_months: i64) -> (i32, u32, u32) {
    // Convert to zero-based month count
    let total = year as i64 * 12 + (month as i64 - 1) + offset_months;
    // compute new year and month
    let new_year = (total / 12) as i32;
    let mut new_month = (total % 12) as i32 + 1;
    if new_month <= 0 {
        new_month += 12;
    }
    let new_month_u = new_month as u32;
    // clamp day to last day of new month
    let max_day = days_in_month(new_year, new_month_u);
    let new_day = if day > max_day { max_day } else { day };
    (new_year, new_month_u, new_day)
}

#[cfg(all(test, feature = "chrono"))]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let dt = DateTime::now();
        assert!(dt.inner.timestamp() > 0);
    }

    #[test]
    fn test_from_iso() {
        let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
        assert_eq!(dt.inner.year(), 2025);
        assert_eq!(dt.inner.month(), 10);
        assert_eq!(dt.inner.day(), 29);
    }

    #[test]
    fn test_plus() {
        let dt = DateTime::from_iso("2025-10-29T00:00:00Z").unwrap();
        let dur = Duration::from_object(&[("days", 3)]);
        let result = dt.plus(&dur);
        assert_eq!(result.inner.day(), 1); // Nov 1
    }

    #[test]
    fn test_start_of() {
        let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
        let start = dt.start_of("day");
        assert_eq!(start.inner.hour(), 0);
        assert_eq!(start.inner.minute(), 0);
    }

    #[test]
    fn test_end_of() {
        let dt = DateTime::from_iso("2025-10-29T14:30:00Z").unwrap();
        let end = dt.end_of("day");
        assert_eq!(end.inner.hour(), 23);
        assert_eq!(end.inner.minute(), 59);
    }
}
