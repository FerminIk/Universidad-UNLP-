package ejercicio1;

import tp02.ejercicio1.ListaDeEnterosEnlazada;

public class TresX {
	private ListaDeEnterosEnlazada l = new ListaDeEnterosEnlazada();
	private int n;
	
	public TresX(int n) {
		super();
		this.n = n;
	}

	public int getN() {
		return n;
	}

	public void setN(int n) {
		this.n = n;
	}

	public ListaDeEnterosEnlazada calcularSucesion (int n) {
		l.agregarFinal(n);
		if (n != 1) {
			if (n%2 == 0) {
				int aux = n/2;
				calcularSucesion(aux);
			} else {
				int aux = 3*n+1;
				calcularSucesion(aux);
			}
		}
		return this.l;
	}
}
