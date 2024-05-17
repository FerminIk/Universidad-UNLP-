package punto2;

public class clase {
	private int[] v;
	private int n;

	public clase(int n) {
		this.n = n;
	}
	
	public int[] crearVector() {
		int[] aux;
		aux = vectorN(this.n);
		
		return aux;
	}
	
	private int[] vectorN(int n) {
		v = new int[n];
		
		for (int i = 0;i<n;i++) {
			v[i] = (int) Math.pow(n, i+1) ;
		}
		
		return v;
	}
}
