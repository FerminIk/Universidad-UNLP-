package punto3;

import java.util.Scanner;
import java.util.Random;

public class main {

	public static void main(String[] args) {
		
		Alumno a = new Alumno("a","a", "a" ,"a.com",1);
		Alumno a1 = new Alumno("b","b", "b" ,"b.com",2);
		Profesor p = new Profesor("c","c", "c" ,"c.com","3");
		Profesor p1 = new Profesor("d","d", "d" ,"d.com","4");
		Profesor p2 = new Profesor("e","e", "e" ,"e.com","5");
		
		Scanner s = new Scanner(System.in);
		Random r = new Random();
		
		System.out.println(a.tusDatos());
		System.out.println(a1.tusDatos());
		
		System.out.println(p.tusDatos());
		System.out.println(p1.tusDatos());
		System.out.println(p2.tusDatos());
	}

}
