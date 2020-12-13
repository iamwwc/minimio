// Token可以为每个fd绑定一个数值，方便调用者区分这个fd的类型
pub struct Token(pub usize);
impl From<Token> for usize {
    fn from(val: Token) -> usize {
        val.0
    }
}