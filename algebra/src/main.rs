use std::io;
fn sorting(array: &mut [isize]) {
    let mut counter = 1;
    while counter < array.len() {
        let mut i = counter;
        while i > 0 {
            if array[i] < array[i - 1] {
                array.swap(i, i - 1);
            }
            i -= 1;
        }
        counter += 1;
    }
}
fn function (array: &Vec<isize>, numb: &isize) -> isize {
    let mut a:u32=0;
    let mut result: isize =0;
    for i in array {
        result+=numb.pow(7-a)*i;
        a+=1;
    }
    result
}

fn print_pol(a: &isize, b:&isize, c:&isize, d:&isize, array: &Vec<isize>, check: &Vec<isize>) {
    let first=b-a;
    let second =(c-2*b+a)/2;
    let third=(d-3*c+3*b-a)/6;
    let three = third;
    let two = second-(array[0]+array[1]+array[2])*third;
    let one = first-(array[0]+array[1])*second+(array[0]*array[1]+array[1]*array[2]+array[0]*array[2])*third;
    let zero =a-array[0]*first+array[0]*array[1]*second-array[0]*array[1]*array[2]*third;

    if (three==check[0] && two==check[1] && one==check[2] && zero==check[3]) || (three==check[0] && two==-1*check[1] && one==-1*check[2] && zero==-1*check[3]) {
        println!("{} {} {} {} <- this it the One", three, two, one, zero);
    } else {
        println!("{} {} {} {}", three, two, one, zero);
    }
}

fn deviders(numb: isize) -> Vec<isize> {
    let mut values: Vec<isize> = Vec::new();
    let mut counter: isize = 2;
    values.push(1);
    values.push(-1);
    if numb>1 {
        values.push(numb);
        values.push(-1*numb);
    }
    while counter*counter<=numb {
        if numb%counter==0 {
            values.push(counter);
            values.push(numb/counter);
            values.push(-1 * counter);
            values.push(-1 * numb / counter);

        }
        counter+=1;
    }
    if counter*counter==numb {
        values.push(counter);
    }
    values
}

fn main() {
    println!("Введите коэфиценты исходного многочлена через пробел:");
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("impossible");
    let array_coef: Vec<isize> = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<isize>().expect("not a number"))
        .collect();

    println!("Введите 4 последовательных целых числа:");
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("impossible");
    let array_knots: Vec<isize> = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<isize>().expect("not a number"))
        .collect();
    let mut values: Vec<isize> = Vec::new();
    for i in &array_knots {
        values.push(function(&array_coef, i));
    }

    println!("Введите коэффициенты ожидаемого многочлена: ");
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("impossible");
    let array_expected: Vec<isize> = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<isize>().expect("not a number"))
        .collect();

    let mut array0 = deviders(values[0].abs());
    let mut array1 = deviders(values[1].abs());
    let mut array2 = deviders(values[2].abs());
    let mut array3 = deviders(values[3].abs());

    sorting(&mut array0);
    sorting(&mut array1);
    sorting(&mut array2);
    sorting(&mut array3);
    let mut count=0;
    println!("var_numb // y0 y1 y2 y3 // Newton polynomial");
    for i in &array0 {
        for u in &array1 {
            for y in &array2 {
                for s in &array3 {
                    if array_coef[0]==6 {
                        if (s - 3 * y + 3 * u - i == 6 || s - 3 * y + 3 * u - i == 12 || s - 3 * y + 3 * u - i == 18 || s - 3 * y + 3 * u - i == 36) && (s - i) % 3 == 0 {
                            count += 1;
                            print!("{} // {} {} {} {} // ", count, i, u, y, s);
                            print_pol(i, u, y, s, &array_knots, &array_expected);
                        }
                    } else {
                        if (s - 3 * y + 3 * u - i == 6 || s - 3 * y + 3 * u - i == 12 || s - 3 * y + 3 * u - i == 24) && (s - i) % 3 == 0 {
                            count += 1;
                            print!("{} // {} {} {} {} // ", count, i, u, y, s);
                            print_pol(i, u, y, s, &array_knots, &array_expected);
                        }
                    }
                }
            }
        }
    }


}
