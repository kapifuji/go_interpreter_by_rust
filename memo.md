以下、コード改善のための参考

字句解析では文字の現在位置が、構文解析では現在のトークン位置がグローバルかつ、それを進める関数は戻り値を取らずに副作用として文字・トークンを進める、というふうになっているが、これはわかりづらい。

字句解析なら現在のトークンと残りの文字列を返す、構文解析なら各parse関数が現在のASTと残りのトークンの列を返す、とすれば副作用がなく可読性が上がる。

⇒現状は、字句解析で現在トークン、構文解析でASTを返しているので、それ以外を対応すればよい。

先読みとか対応するのはややこしそうだが...
