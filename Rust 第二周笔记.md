# Rust 第二门课笔记



## 所有权



### 变量的所有权

对于固定字节数的类型，默认会保存在栈上；对于不固定字节数的类型，默认创建在堆，然后用栈上的局部变量指向它。

在Java的赋值语句中，固定字节类型的赋值通常会把值重新拷贝一份给另一变量，因为占用内存小；但对于不固定字节的复杂类型，则通常会把地址拷贝给另一变量，否则占用内存大，非常低效。

在Rust中，固定字节类型同Java；但对于不固定字节的复杂类型，赋值相当于移动。移动后，原变量不可用。

```rust
fn main () {
	let s1 = String::from ("I am a superman.");	//String占用字符数不定，创建在堆。左侧s1创建于栈上，然后指向右侧的String。
  let s2 = s1;	//s1被移动到了s2。s1被移动后不能使用。
  printIn! ("{s1}"); //s1被使用了，故报错。
  println! ("{s2}");
}
```



在rust中，值被看作是资源，所有权指的就是对资源的控制权。rust对所有权有三条定义：

​	1&2.每一个资源有且只有一个所有者

​	3.所有者所在的作用域结束时，就会把资源/值释放掉。



所以上方例子中，s1的所有权被转移了，所以s1就不可使用，被悬置起来了，直到最后在<font color='red'>“ } ”</font>处释放。



这种特性被称为RAII（Resource Acquisition Is Initialization）。



### 所有权与函数域

函数作用域是能够释放所有权对应资源的。

```rust
fn foo (s: String){
  printin! ("{s}");
}

fn main () {
  let s1 = String::from ("I am a superman.");
  foo (s1); //函数参数s获得了s1的所有权，在函数调用结束后，s对应的资源被释放。s1处于无效状态。
  printin! ("{s1}");//s1不可用，报错。
}
```



由于函数执行完后，资源随着函数作用域一同被释放掉，所以想要保留资源，就需要在函数作用域内把资源所有权返回。如下：

```rust
fn foo (s: String)->String{
  printin! ("{s}");
  s
}

fn main () {
  let s1 = String::from ("I am a superman.");
  let s1 = foo (s1); 
  printin! ("{s1}");
}
```



## 可变引用&&不可变引用



### 右侧为引用地址的赋值语句

#### 1.对于固定字符类型的引用

引用是固定长度的值，所以赋值语句也是将值复制给另一变量。

rust中对同一个地址进行多次引用会自动忽略中间引用。

效果是地址的复制。



#### 2.对于非固定字符类型的引用

效果是所有权的借用，资源地址没有发生变化。

```rust
fn main () {
  let s1 = String:: from ("I am a superman.");
  
  //引用s1的所有权。
  //所有语句中的变量都指向同一个资源，资源没有复制，所有权页没有转移，只是产生了多个不可用引用变量。
  let s2 = &s1;
  let s3 = &&&&&s1;
  let s4 = &s2:	
  let s5 = s2;
  
  //不可变引用作用域重叠。
  printin! ("{s1}");	
  printin! ("{s2}");
  printin! ("{s3}");
  printin! ("{s4}");
  printIn! ("{s5}");
}
```



#### 3/4.可变引用的赋值语句 && 可变引用对资源的修改

```rust
fn main () {
  let mut a = 10u32;//a可变。
  let b = &mut a; //引用a，且能够修改所有权对应值。
  *b = 20;//解b的所有权，访问对应值并修改。
  printin! ("{b}");
}
```

（类似指针）



#### 5.引用作用域的叠加限制性

引用规则：

​	1.引用的作用域是从它的定义到它的最后一次使用时结束。

​	2.一个资源的可变引用和不可变引用的作用域不能够交叠。

​	3.一个资源只能同时存在一个可变引用。

​	3.一个资源的不可变引用可以同时存在多个。

```rust
fn main (){
  let mut a = 10u32;
  let b = &mut a;
  *b = 20;
  
  println!("{a}");	//在可变引用b的作用域内有可变引用a，报错。
  println!("{b}");
}
```

```rust
fn main () {
  let mut a = 10u32;
  let b = &mut a;
  *b = 20;
  let c = &a; //在可变引用b的作用域内有不可变引用c，报错。
  println! ("{b}");
}
```



### 右侧为引用变量的赋值语句



#### 1.不可变引用变量的赋值语句

不可变引用变量的赋值语句效果为复制。



#### 2.可变引用变量的赋值语句

可变引用变量的赋值语句效果为移动。

```rust
fn main (){
  let mut a = 10u32;
  let r1 = &mut a;
  let r2 = r1;//r1的可变引用移动给了r2，r1失效。
  printin! ("{r1}");//报错
}
```



### 两种引用与函数传入参与



#### 1.函数参数为不可变引用

传入参数为不可变引用时，不可需改对应资源，但所有权没有变化，因此函数执行结束后，传入参数变量对应资源依然有效。

```rust
fn foo (s: &String){	//s参数为String类型不可变引用
println! ("in fn foo: {s}");
}

fn main () {
let s1 = String::from ("I am a superman.");
foo (&s1) ; //传入String类型不可变引用
printin! ("{s1}"); //s1仍然有效
}
```



#### 2.函数参数为可变引用

传入参数为可变引用时，可修改引用对应资源。

```rust
fn foo(s: &mut string){	//s参数为String类型可变引用
s.push_str(" You are batman.") ;	//将会为任何传入String参数后面增加字符串字面值
}

fn main () {
let mut s1 = String::from ("I am a superman."); //定义可变所有权
printin! ("{s1}");
foo (&mut s1); //传入可变引用
println! ("{s1}");	//s1的所有权没有移动，且值已变更
```





