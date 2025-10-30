use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Duration {
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
}

impl Duration {
    pub fn from_object(obj: &[(&str, i64)]) -> Self {
        let mut dur = Duration::default();
        for (key, value) in obj {
            match *key {
                "years" | "year" => dur.years = *value,
                "months" | "month" => dur.months = *value,
                "weeks" | "week" => dur.weeks = *value,
                "days" | "day" => dur.days = *value,
                "hours" | "hour" => dur.hours = *value,
                "minutes" | "minute" => dur.minutes = *value,
                "seconds" | "second" => dur.seconds = *value,
                "milliseconds" | "millisecond" => dur.milliseconds = *value,
                _ => {}
            }
        }
        dur
    }

    pub fn to_object(&self) -> HashMap<String, i64> {
        let mut map = HashMap::new();
        if self.years != 0 {
            map.insert("years".to_string(), self.years);
        }
        if self.months != 0 {
            map.insert("months".to_string(), self.months);
        }
        if self.weeks != 0 {
            map.insert("weeks".to_string(), self.weeks);
        }
        if self.days != 0 {
            map.insert("days".to_string(), self.days);
        }
        if self.hours != 0 {
            map.insert("hours".to_string(), self.hours);
        }
        if self.minutes != 0 {
            map.insert("minutes".to_string(), self.minutes);
        }
        if self.seconds != 0 {
            map.insert("seconds".to_string(), self.seconds);
        }
        if self.milliseconds != 0 {
            map.insert("milliseconds".to_string(), self.milliseconds);
        }
        map
    }

    pub fn as_unit(&self, unit: &str) -> i64 {
        let total_ms = self.as_milliseconds();
        match unit {
            "milliseconds" | "millisecond" => total_ms,
            "seconds" | "second" => total_ms / 1000,
            "minutes" | "minute" => total_ms / (1000 * 60),
            "hours" | "hour" => total_ms / (1000 * 60 * 60),
            "days" | "day" => total_ms / (1000 * 60 * 60 * 24),
            "weeks" | "week" => total_ms / (1000 * 60 * 60 * 24 * 7),
            "months" | "month" => total_ms / (1000 * 60 * 60 * 24 * 30),
            "years" | "year" => total_ms / (1000 * 60 * 60 * 24 * 365),
            _ => 0,
        }
    }

    pub(crate) fn as_milliseconds(&self) -> i64 {
        let mut ms = self.milliseconds;
        ms += self.seconds * 1000;
        ms += self.minutes * 60 * 1000;
        ms += self.hours * 60 * 60 * 1000;
        ms += self.days * 24 * 60 * 60 * 1000;
        ms += self.weeks * 7 * 24 * 60 * 60 * 1000;
        ms += self.months * 30 * 24 * 60 * 60 * 1000;
        ms += self.years * 365 * 24 * 60 * 60 * 1000;
        ms
    }

    pub(crate) fn components(&self) -> (i64, i64, i64, i64, i64, i64, i64, i64) {
        (
            self.years,
            self.months,
            self.weeks,
            self.days,
            self.hours,
            self.minutes,
            self.seconds,
            self.milliseconds,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_object() {
        let dur = Duration::from_object(&[("days", 3), ("hours", 2)]);
        assert_eq!(dur.days, 3);
        assert_eq!(dur.hours, 2);
    }

    #[test]
    fn test_to_object() {
        let dur = Duration::from_object(&[("weeks", 1), ("minutes", 30)]);
        let obj = dur.to_object();
        assert_eq!(obj.get("weeks"), Some(&1));
        assert_eq!(obj.get("minutes"), Some(&30));
    }

    #[test]
    fn test_as_unit() {
        let dur = Duration::from_object(&[("hours", 2)]);
        assert_eq!(dur.as_unit("minutes"), 120);
        assert_eq!(dur.as_unit("seconds"), 7200);
    }
}
