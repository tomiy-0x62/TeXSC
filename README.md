# TeXSC: TeX Scientific Calculator

インタラクティブとコマンドライン, ファイルからの読み込みで使えるTeXの数式を計算できる計算機

## Installation
```
git clone https://github.com/tomiy-0x62/TeXSC
cd TeXSC
# install to ~/.cargo/bin
cargo install --path .
# or install to preferred directory
cargo build --release
cp target/release/tsc /path/to/preferred_directory
```

### Requirements
+ Rust toolchain (install using rustup)

## Configuration

+ On Unix system (Linux, macOS):
`$HOME/.config/tsc/config.toml`

+ On Windows:
`%APPDATA%\tsc\config.toml`

config.tomlの例
```
debug = false
ast_format = "Tree"
trig_func_arg = "Radian"
log_base = 2.718281828459045
num_of_digit = 8
```

## Usage

インタラクティブ
```
$ tsc
tsc> 1 + 1
2
tsc> :q
$
```
コマンドライン
```
$ tsc "1 + 1"
2
```
ファイル
```
$ cat hoge.txt
1+1
$ tsc -f hoge.txt
2
```
どれでも変数が使える
```
$ tsc
tsc> ;a = 1
tsc> a + 1
2
or
$ tsc "a + 1 ;a = 1"
2
or 
$ cat hoge.txt
;a=1
a+1
$ tsc -f hoge.txt
2
```

## 数値リテラル
10進数, 16進数(prefix: 0x), 8進数(prefix: 0), 2進数(prefix: 0b), 科学的表記(E表記)(e.g. -8.984E+8, 1.234E-5)に対応

### 10進数
先頭と末尾以外に桁の区切りを示すために`,`または`_`を入れることができる
```
12345
12_345
12,345
1.234,56
12.345,6
123,456.32_2
```

### 16進数
先頭と末尾以外に桁の区切りを示すために`_`を入れることができる
```
0x1234
0x12_34
```

### 8進数
先頭と末尾以外に桁の区切りを示すために`_`を入れることができる
```
01234
012_34
```

### 2進数
先頭と末尾以外に桁の区切りを示すために`_`を入れることができる
```
0b11000011
0b1100_0011
```

### 科学的表記(E表記)
桁の区切りを示すためのセパレータ`,`または`_`は使用できない

## 機能
### TeXコマンド
#### 対数
`\log`のデフォルトの底はネイピア数`e`

`:logbase {num(f64)}`で変更可能
- `\log x` : 対数
- `\ln x` : 自然対数(底がネイピア数`e`の対数)
```
tsc> \log 4
1.3862943611198906
```
#### ルート
- `\sqrt{x}` : 平方根
```
tsc> \sqrt {4}
2
```
#### 分数
- `\frac{a}{b}`
```
tsc> \frac{3}{4}
0.75
```
#### 絶対値
- `\abs (x)`
```
tsc> \abs(-2)
2
```
#### 三角関数
デフォルトでは、引数はラジアンとして解釈される

`:trarg {rad|deg}`で三角関数の引数の解釈を変更できる
- `\sin x`
- `\cos x`
- `\tan x`
- `\csc x`
- `\sec x`
- `\cot x`
- `\arcsin x`
- `\arccos x`
- `\arctan x`
```
tsc> \sin 1
0.8414709848078965
```
#### その他
- `^{x}`
- `\exp (x)`
```
tsc> 2^{10}
1024
tsc> \exp(3)
20.085536923187664
```
#### 2項四則演算
- `a + b`
- `a - b`
- `a * b`
- `a \times  b` : 掛け算
- `a \cdot b` : 掛け算
- `a / b`
- `a \div b` : 割り算
```
tsc> 6 * 7
42
```

#### 単項演算子
- `-`
```
tsc> -3
-3
```

### TSCコマンド
TSCに対するコマンド

`:`で始まる
#### `:q`
TSCを終了

#### `:help`
TSCコマンドに関するヘルプを表示

#### `:debug {true|false}`
デバッグモードの切り替え

#### `:logbase {num(f64)}`
`\log`の底の変更

#### `:rlen {num(u32)}`
計算結果の表示の有効数字の変更

#### `:trarg {rad|deg}`
三角関数の引数の解釈を変更

#### `:astform {tree|sexpr|both|none}`
ASTの表示形式の変更

#### `:write conf`
現在の設定をconfig.tomlへ書き込み

#### `:reload conf`
config.tomlの再読み込み

#### `:hex {tex formulas} ...`
このコマンド以降の式の値を16進数表記で表示

例:
```
tsc> :hex 0x42 42 0b1101 012
0x42
0x2a
0xd
0xa
tsc> 0x42 :hex 42
66
0x2a
```

#### `:dec {tex formulas} ...`
このコマンド以降の式の値を10進数表記(有効数字無視)で表示

例:
```
tsc> :dec 0x42 42 0b1101 012
66
42
13
10
tsc> 42 :dec 0x42
42
66
```

#### `:bin {tex formulas} ...`
このコマンド以降の式の値をoctet単位に0-padされた2進数表記で表示

例:
```
tsc> :bin 0x42 42 0b1101 012
0b01000010
0b00101010
0b00001101
0b00001010
tsc> :bin 0x1234
0b00010010_00110100
tsc> 42 :bin 0x42
42
0b01000010
```

#### `:oct {tex formulas} ...`
このコマンド以降の式の値を8進数表記で表示

例:
```
tsc> :oct 0x42 42 0b1101 012
0102
052
015
012
tsc> 042 :oct 42
34
052
```

#### `:fact {num(u64)}`
数字を素因数分解

例:
```
tsc> :fact 42
42 = 2 * 3 * 7
```

#### `:gcd {num(u64)} {num(u64)} ...`
最大公約数を計算

例:
```
tsc> :gcd 12 42 66
gcd(12, 42, 66) = 6
```

#### `:redu {num(u64)} {num(u64)} ...`
最大公約数で割る

例:
```
tsc> :redu 12 42 66
12 : 42 : 66 = 2 : 7 : 11
```

#### `:show {var|const|config|conf}`
変数、設定、組み込み定数を表示



### 定数
- `\pi` : 円周率
- `e` : ネイピア数
```
tsc> \pi
3.141592653589793
tsc> e
2.718281828459045
```

### セパレータ
複数の式を入力する場合セパレータ`;`で明示的に式を分割できる
```
tsc> \sin \frac{\pi}{2} x^{2} ;x=3
9
tsc> \sin \frac{\pi}{2}; x^{2} ;x=3
1
9
```

### 変数
変数の宣言は"; a = 3"の形式で宣言する

ファイルからの入力やインタラクティブでは1度変数を宣言するとTeXSCを終了するまで変数の値は保持される

同じ名前の変数を別の値で宣言すると、変数の値が上書きされる

1行に数式と変数宣言の両方を含めた場合、数式と変数宣言の順序は自由

変数名に使えるのは正規表現`r"[A-Za-z][A-Za-z0-9]\*"`にマッチするもの
```
tsc> 6+x ; x = 36
42
tsc> ;a = 3 5/a
1.6666666666666667
```

### 計算結果の有効数字
デフォルトでは12桁

`:rlen {num(u32)}`で変更

0に設定した場合、有効数字を無視した結果が表示される

### ASTの表示
以下に示すtree形式もしくはS式としてASTが表示される

`:astform {tree|sexpr|both|none}`で表示形式を切り替えられる
#### tree
```
{演算子}
├──{左 operand}
└──{右 operand}
```
例: `\frac{4}{3+2}`
```
/
├──4
└──+
   ├──3
   └──2
```
#### S式
sbclで評価可能なS式が出力される
```
({演算子} {左 operand} {右 operand})
```
例: `\frac{4}{3+2}`
```
(/ 4 (+ 3 2) )
```

### デバッグモード
`dev` profileでビルドされている場合、もしくは`debug`が`true`になっている場合、デバッグ用の情報が出力される

`:debug {true|false}`でモードの切り替え
