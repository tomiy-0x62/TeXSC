# TeXSC: TeX Scientific Calculator

インタラクティブとコマンドライン, ファイルからの読み込みで使えるTeXの数式を計算できる計算機

## Usage

インタラクティブ
```
$ tsc
tsc> 1 + 1
2
```
コマンドライン
```
$ tsc "1 + 1"
2
```
ファイル
```
$cat hoge.txt
1+1
$ tsc -f hoge.txt
2
```
どれでも変数が使える
```
$ tsc
tsc> ,a = 1
tsc> a + 1
2
or
$ tsc "a + 1 ,a = 1"
2
or 
$ cat hoge.txt
,a=1
a+1
$ tsc -f hoge.txt
2
```

## 機能
### TeXコマンド
#### 対数
- \log x : 自然対数
- \ln x : 自然対数
```
tsc> \log 4
1.3862943611198906
```
#### ルート
- \sqrt{x} : 平方根
```
tsc> \sqrt {4}
2
```
#### 分数
- \frac{a}{b}
```
tsc> \frac{3}{4}
0.75
```
#### 絶対値
- \abs (x) 　// 未実装
```
tsc> \abs(-2)
2
```
#### 三角関数 (引数はラジアン)
- \sin x
- \cos x
- \tan x
- \csc x
- \sec x
- \cot x
```
tsc> \sin 1
0.8414709848078965
```
#### その他
- ^{x}   // 未実装
- \exp (x)
```
tsc> 2^{10}
1024
tsc> \exp(3)
20.085536923187664
```
#### 2項四則演算
- a + b
- a - b
- a * b
- a \times  b : 掛け算
- a \cdot b : 掛け算
- a / b
- a \div b : 割り算
```
tsc> 6 * 7
42
```

### 定数
- \pi : 円周率
- e : ネイピア数
```
tsc> \pi
3.141592653589793
tsc> e
2.718281828459045
```

2
