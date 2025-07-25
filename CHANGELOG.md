# Change Log

## [0.3.1](https://github.com/tomiy-0x62/TeXSC/releases/tag/0.3.1) (2025-07-25)

**Implemented enhancements:**

+ add full support for `\log` and `\ln` in S-expr AST

+ add support for `\csc`, `\sec` and `\cot` in S-expr AST

**Fixed bugs:**

+ Fix incorrect Arc{Sin|Cos|Tan} calculations when trig_func_arg is set to degrees

## [0.3.0](https://github.com/tomiy-0x62/TeXSC/releases/tag/0.3.0) (2025-07-24)

**Implemented enhancements:**

+ use BigDecimal instead of f64

## [0.2.4](https://github.com/tomiy-0x62/TeXSC/releases/tag/0.2.4) (2025-02-20)

**Implemented enhancements:**

+ add new tsc commands `:fact`, `:gcd`, `:redu`

## [0.2.3](https://github.com/tomiy-0x62/TeXSC/releases/tag/0.2.3) (2024-09-07)

**Implemented enhancements:**

+ change how to quit tsc (`exit` -> `:q`)

+ add new tsc commands `:hex`, `:dec`, `:bin` 

+ don't show config load message when formulas form file or command line arg

**Fixed bugs:**

+ Fix deadlock bug when formulas from file or command line arg

## [0.2.2](https://github.com/tomiy-0x62/TeXSC/releases/tag/0.2.2) (2024-06-19)

**Implemented enhancements:**

+ change inverse trigonometric funstions (Arc{sin/cos/tan}) result format same as trig_func_arg

**Fixed bugs:**

+ Fix ast_format config not work properly

## [0.2.1](https://github.com/tomiy-0x62/TeXSC/releases/tag/0.2.1) (2024-05-24)

**Implemented enhancements:**

+ save current config to file, and load config from file [#19](https://github.com/tomiy-0x62/TeXSC/issues/19)

+ Fix wrong error message
