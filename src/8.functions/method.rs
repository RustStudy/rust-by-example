/*
方法是属于对象的函数。
这些方法可以访问对象的数据，以及通过self关键字来访问其他方法。
方法都被定义在impl块下面。
*/

struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` methods go in here
// 实现块，所有`Point`的方法都在这里
impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    // 这是静态方法
    // 静态方法不需要被实例调用
    // 这些方法通常被用作构造函数
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, that takes two arguments
    // 另外的静态方法，带两个参数
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    //  这是实例方法
    // `&self`是`self: &Self`的语法糖， `Self`是调用对象的类型。
    // 在这个例子里`Self`=`Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self` 可以通过点操作访问结构体的字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // 该方法要求调用对象是可变的。
    // `&mut self`是`self: &mut Self`的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`拥有的资源：两个堆分配的数字
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // 该方法“消费”调用对象的资源
    // `self`是`self: Self`的语法糖
    fn destroy(self) {
        // Destructure `self`
        // 解构`self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`和`second`出了作用域外就会被释放
    }
}

fn main() {
    let rectangle = Rectangle {
        // Static methods are called using double colons
        // 静态方法使用双冒号来调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Instance method are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `perimeter(&rectangle)`
    // 实例方法使用点操作来调用
    // 注意第一个参数是`&self`被隐式传递的
    // `rectangle.perimeter()` 等价于 `perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // 错误， `rectangle`是不可变的，但是该方法需要可变对象
    // rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line

    // Ok, mutable object can call mutable methods
    // 可变对象能调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // 错误！ 前面的`destroy`调用“消费”了`pair`
    // 注： 所有权被转移
    // pair.destroy();
    // TODO ^ Try uncommenting this line
}
