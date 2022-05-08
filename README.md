# TeXSC: TeX Scientific Calculator

インタラクティブとコマンドライン, ファイルからの読み込みで使える
インタラクティブ
$ tsc
tsc> 1 + 1
2
コマンドライン
$ tsc "1 + 1"
2
ファイル
$cat hoge.txt
1+1
$ tsc -f hoge.txt
2
どれでも変数が使える
$ tsc
tsc> a = 1
tsc> a + 1
2
or
$ tsc "a + 1",  "a = 1"
2
or 
$ cat hoge.txt
a+1
a=1
$ tsc -f hoge.txt
2

## 対応(予定)の機能
### TeXコマンド
### 対数
- \log : 底が2の対数
- \log_a : 底がaの対数
- \ln : 自然対数 
### ルート
- \sqrt{x} : 平方根
- \sqrt[n]{x} : n乗根
### 分数
- \frac{a}{b}
### 絶対値
- \abs
- |
- \left|
- \right|
### 三角関数
- \sin
- \cos
- \tan
- \csc
- \sec
- \cot
### その他
- ^{}
- _{}
- ! (階乗)
- \exp
### 四則演算
- +
- -
- \times : 
- \cdot :
- \div :
- / :
### 和, 積
- \sum_{a}^{n=1}
- \prod

## 対応定数
- e: ネイピア数
- \pi: 円周率

### usage
$ tsc "1+1"
2
$ tsc "\frac{3}{4}"
0.75
$ tsc "sin\frac{3}{4}\pi"
0.4111176183
$ tsc "\log_3 9"
2

方針: 
tokenに分割 1 + 1 > "1", "+", "1"
パースした後スタックを使って計算
step1: 1 + 1, 3 * 4
step2: 1^2, 1^{2}
step3: \frac{3}{4}
