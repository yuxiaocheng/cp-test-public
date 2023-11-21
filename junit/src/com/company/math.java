package com.company;

public class math {
    public int factorial(int n) throws Exception {
        if(n<0){
            throw new Exception("负数没有阶乘");
        }else if(n<=1){
            return 1;
        }else{
            return n * factorial(n-1);
        }
    }

    public int add(int x, int y) { //加法
        return x + y;
    }
}
