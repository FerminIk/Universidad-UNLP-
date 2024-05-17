package punto1;

public class clase1 {
	
	private int a;
	private int b;
	
	public clase1(int a, int b) {
		this.a = a;
		this.b = b;
	}
	
	public void setA(int a) {
		this.a = a;
	}
	
	public int getA(int a) {
		return a;
	}
	
	public void setB(int b) {
		this.b = b;
	}
	
	public int getB(int b) {
		return b;
	}
	
	public void conteoFor()  {
		for (int i = this.a ; i-1 < this.b ; i++) {
			System.out.println(i);
		}
	}
	
	public void conteoWhile()  {
		int aux = this.a;
		while (aux-1 != this.b) {
			System.out.println(aux);
			aux++;
		}
	}
	
	public void conteoRecursivo() {
		conteoRecursivoPriv(this.a,this.b);
	}
	
	private void conteoRecursivoPriv(int x, int y)  {
		if (x <= y) {
			System.out.println(x);
			conteoRecursivoPriv(x+1,y);
		}
	}
}