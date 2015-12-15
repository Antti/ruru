use types;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_id2sym(id: types::rb_id) -> types::rb_value;
    pub fn rb_id2name(id: types::rb_id) -> *const types::c_char;
    pub fn rb_sym2id(id: types::rb_value) -> types::rb_id;

    pub fn rb_intern(name: *const types::c_char) -> types::rb_id;
    pub fn rb_const_get(klass: types::rb_value, id: types::rb_value) -> types::rb_value;
    pub fn rb_funcallv(receiver: types::rb_value,
                       method: types::rb_value,
                       argc: types::c_int,
                       argv: *const types::rb_value) -> types::rb_value;
}
