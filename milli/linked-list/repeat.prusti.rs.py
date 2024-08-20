# def push_code(n):
#     ens_lookup = lambda i: """
#     #[ensures(snap(self.lookup({i})) === elem)]""".format(i=i)
#     signature = ("""
#     #[ensures(self.len() == old(self.len()) + {n})]""".format(n=n) +
#     "".join(ens_lookup(i) for i in range(n)) + """
#     #[ensures(forall(|i: usize| (i < old(self.len())) ==>
#         old(self.lookup(i)) === self.lookup(i + {n})))]
#     pub fn push{n}(&mut self, elem: T) {{""".format(n=n))
#     
# 
#     assert_lookup = lambda k: """
#     prusti_assert!(snap(self.lookup({k})) === elem);""".format(k=k)
#     body = lambda j: ("""
#     let new_node = Box::new(Node {{
#         elem: elem.clone(),
#         next: self.head.take(),
#     }});
#     self.head = Some(new_node);
#     prusti_assert!(self.len() == old(self.len()) + {j});""".format(j=j) +
#     "".join(assert_lookup(i) for i in range(j)) + """
#     prusti_assert!(forall(|i: usize| (i < old(self.len())) ==>
#         old(self.lookup(i)) === self.lookup(i + {j})));""".format(j=j))
# 
#     f = signature + "".join(body(i + 1) for i in range(n)) + """
#     }
# """
#     return f

def main_code(n):
    p = lambda i: """
    l1.push({i});
    l2.push({i});
    l3.push({i});
    l4.push({i});
    """.format(i=i)
    return ("""
fn main() {
    let mut l1 = LinkedList::<u64>::new();
    let mut l2 = LinkedList::<u64>::new();
    let mut l3 = LinkedList::<u64>::new();
    let mut l4 = LinkedList::<u64>::new();
    """ +
    "".join(p(i) for i in range(n)) + """
}"""
    )