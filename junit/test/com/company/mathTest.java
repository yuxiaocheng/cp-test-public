package com.company;

import static org.junit.Assert.*;
import org.junit.Before;
import org.junit.Test;

public class mathTest {
    @Before
    public void setup() throws Exception{

    }
    @Test
    public void factorial1() throws Exception{
        assertEquals(120,new math().factorial(5));
    }

    @Test (expected = Exception.class)//看测试代码是否抛出了想要得到的异常

    public void testFactorialException() throws Exception {
        new math().factorial(-1);
        fail("factorial参数为负数没有抛出异常");
    }
    @Test
    public void factorial2() throws Exception{
        assertEquals(24,new math().factorial(4));
    }
    @Test
    public void factorial3() throws Exception{
        assertEquals(6,new math().factorial(3));
    }
    @Test
    public void factorial4() throws Exception{
        assertEquals(720,new math().factorial(6));
    }



    @Test
    public void factorial5() throws Exception{
        assertEquals(220,new math().factorial(7));
    }
    @Test
    public void factorial6() throws Exception{
        assertEquals(120,new math().factorial(43));
    }
    @Test
    public void factorial7() throws Exception{
        assertEquals(4230,new math().factorial(9));
    }
    @Test
    public void factorial8() throws Exception{
        assertEquals(2222,new math().factorial(32));
    }
    @Test
    public void factorial9() throws Exception{
        assertEquals(1240,new math().factorial(42));
    }
    @Test
    public void factorial10() throws Exception{
        assertEquals(13,new math().factorial(542));
    }
    @Test
    public void factorial11() throws Exception{
        assertEquals(1240,new math().factorial(54));
    }



}