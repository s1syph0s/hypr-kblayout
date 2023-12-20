#[derive(Debug, PartialEq, Eq)]
pub struct KeyboardConfig<'a> {
    keyboard_name: &'a str,
    layout: &'a str,
}

impl<'a> KeyboardConfig<'a> {
    pub fn new(raw_str: &'a str) -> Option<Self> {
        if !Self::event_valid(raw_str) {
            return None;
        }
        let Some(data) = Self::get_data(raw_str) else {
            return None;
        };
        
        let Some((keyboard_name, layout)) = data.rsplit_once(',') else {
            return None;
        };

        Some(Self { keyboard_name, layout })
    }

    fn event_valid(raw_str: &'a str) -> bool {
        const KBD_EVENT: &str = "activelayout";
        if let Some(i) = raw_str.find('>') {
            let event = &raw_str[..i];
            if event == KBD_EVENT {
                return true;
            }
        }
        false
    }

    fn get_data(raw_str: &'a str) -> Option<&'a str> {
        if let Some(i) = raw_str.rfind('>') {
            let data = &raw_str[i+1..];
            return Some(data);
        }
        None
    }

    pub fn keyboard_name(&self) -> &str {
        self.keyboard_name
    }

    pub fn layout(&self) -> &str {
        self.layout
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn event_is_valid() {
        let raw_str = "activelayout>>smthing";
        assert!(KeyboardConfig::event_valid(raw_str));
    }

    #[test]
    fn event_invalid() {
        let raw_str = "workspace>>smthing";
        assert!(!KeyboardConfig::event_valid(raw_str));
    }

    #[test]
    fn test_get_data() {
        let raw_str = "activelayout>>smthing,else";
        assert_eq!(Some("smthing,else"), KeyboardConfig::get_data(raw_str));
    }

    #[test]
    fn test_new() {
        let raw_str = "activelayout>>hp,-inc-hyperx-alloy-origins,English (US)";
        let kbd_conf = KeyboardConfig::new(raw_str);
        let target_conf = KeyboardConfig { 
            keyboard_name: "hp,-inc-hyperx-alloy-origins",
            layout: "English (US)",
        };

        assert_eq!(Some(target_conf), kbd_conf);
    }
}
