use crate::json::client::Rule;

impl Rule {
    pub fn is_allowed(&self) -> bool {
        let mut is_matched = true;
        if self.os.is_some() {
            let os = self.os.clone().unwrap();

            if os.name.is_some() {
                if !(os.name.unwrap() == crate::OS) {
                    is_matched = false;
                }
            }

            if os.arch.is_some() {
                if !(os.arch.unwrap() == crate::ARCH) {
                    is_matched = false;
                }
            }
        }

        if is_matched && self.action == "allow" || !is_matched && self.action == "disallow" {
            return true;
        }

        return false;
    }
}

pub fn is_allowed(rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if !rule.is_allowed() {
            return false;
        }
    }

    return true;
}
