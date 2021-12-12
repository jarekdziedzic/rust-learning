pub struct ListNode<'a, T> {
    value : T,
    next : Option<&'a ListNode<'a, T>>,
}

pub struct LinkedList<'a, T> {
    first : Option<&'a ListNode<'a, T>>,
}

impl<'a, T> LinkedList<'a, T> {
    pub fn len(&self) -> u32 {
        let mut lenght = 0;
        if self.first.is_some() {
            lenght += 1;
            let mut elem = self.first.expect("Null list element");
            while elem.next.is_some() {
                lenght += 1;
                elem = elem.next.expect("Null list element");
            }
        }
        lenght
    }
}

pub fn test_lists() {
    let le1 = ListNode { value : 5, next : None };
    let le2 = ListNode { value : 6, next : Some(&le1) };
    let le3 = ListNode { value : 7, next : Some(&le2) };

    let list  = LinkedList::<i32>{ first : Some(&le3) };
    println!("List length is {}", list.len() );

    let list2  = LinkedList::<i32>{ first : Some(&le1) };
    println!("List length is {}", list2.len() );

    let list3  = LinkedList::<i32>{ first : None };
    println!("List length is {}", list3.len() );
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, ListNode};

    #[test]
    fn length_of_empty_list() {
        let list = LinkedList::<i32>{ first : None};
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn length_of_one_element_list() {
        assert_eq!(LinkedList::<i32>{ first : Some(&ListNode{ value: 1, next : None})}.len(), 1);
    }

    #[test]
    fn list_elem_value_can_be_read() {
        let list = LinkedList::<i32>{ first : Some(&ListNode{ value: 1, next : None})};
        let le = list.first.expect("Failed to get list node");
        assert_eq!(le.value, 1);
    }
}