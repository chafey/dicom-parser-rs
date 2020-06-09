use crate::attribute::Attribute;

pub type ConditionFN = fn(&Attribute) -> bool;

pub fn is_not_private(attribute: Option<&Attribute>) -> Option<&Attribute> {
    if let Some(attr) = attribute {
        if !attr.tag.is_private() {
            return attribute;
        }
    }
    None
}

pub fn is_group(group: u16, attribute: Option<&Attribute>) -> Option<&Attribute> {
    if let Some(attr) = attribute {
        if attr.tag.group == group {
            return attribute;
        }
    }
    None
}

pub fn any(_attribute: &Attribute) -> bool {
    true
}

pub fn none(_attribute: &Attribute) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::{is_group, is_not_private};
    use crate::attribute::Attribute;
    use crate::tag::Tag;

    #[test]
    fn is_group_8_8_returns_some() {
        let attr = Attribute {
            tag: Tag {
                group: 8,
                element: 8,
            },
            vr: None,
            length: 0,
            had_unknown_length: false,
        };
        let result = is_group(8, Some(&attr));
        assert!(result.is_some());
    }

    #[test]
    fn is_group_8_2_returns_none() {
        let attr = Attribute {
            tag: Tag {
                group: 8,
                element: 8,
            },
            vr: None,
            length: 0,
            had_unknown_length: false,
        };
        let result = is_group(2, Some(&attr));
        assert!(result.is_none());
    }

    #[test]
    fn is_group_none_returns_none() {
        let result = is_group(2, None);
        assert!(result.is_none());
    }

    #[test]
    fn is_not_private_8_returns_some() {
        let attr = Attribute {
            tag: Tag {
                group: 8,
                element: 8,
            },
            vr: None,
            length: 0,
            had_unknown_length: false,
        };
        let result = is_not_private(Some(&attr));
        assert!(result.is_some());
    }

    #[test]
    fn is_not_private_9_returns_none() {
        let attr = Attribute {
            tag: Tag {
                group: 9,
                element: 8,
            },
            vr: None,
            length: 0,
            had_unknown_length: false,
        };
        let result = is_not_private(Some(&attr));
        assert!(result.is_none());
    }

    #[test]
    fn is_not_private_none_returns_none() {
        let result = is_not_private(None);
        assert!(result.is_none());
    }
}
