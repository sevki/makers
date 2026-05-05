@wrap_lit_add@
expression LEN;
@@

- (1).wrapping_add(LEN)
+ (1 as size_t).wrapping_add(LEN)

@wrap_lit_add_one_more@
expression LEN;
@@

- (1).wrapping_add(LEN).wrapping_add(1)
+ (1 as size_t).wrapping_add(LEN).wrapping_add(1)

@wrap_lit_mul_4@
expression E;
@@

- (4).wrapping_add(E)
+ (4 as size_t).wrapping_add(E)

@wrap_lit_mul_53@
expression E;
@@

- (53).wrapping_mul(E)
+ (53 as size_t).wrapping_mul(E)
