package punto2;

import java.util.Scanner;
public class main {

	public static void main(String[] args) {
		
		Scanner s = new Scanner(System.in);
		
		clase c;
		int[] v;
		int n;
		
		System.out.println("Introduzca un  numero");
		n = s.nextInt();
		c = new clase(n);
		v = c.crearVector();
		
		for (int i=0;i<n;i++) {
			System.out.println("Numero: " + v[i]);
		}
	}

}
