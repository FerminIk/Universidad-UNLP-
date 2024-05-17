package punto4;

import ejercicio1.ArbolBinario;

public class RedBinariaLlena {
	private ArbolBinario<Integer> raiz;

	public RedBinariaLlena(ArbolBinario<Integer> raiz) {
		super();
		this.raiz = raiz;
	}

	public ArbolBinario<Integer> getRaiz() {
		return raiz;
	}

	public void setRaiz(ArbolBinario<Integer> raiz) {
		this.raiz = raiz;
	}
	
	private int maximoRetardo(ArbolBinario<Integer> a) {
		int max = 0;
		if (!a.esVacio()) {
			int izquierda = 0;
			int derecha = 0;
			if (a.tieneHijoIzquierdo()) {
				izquierda = maximoRetardo(a.getHijoIzquierdo()) ;
			}
			if (a.tieneHijoDerecho()) {
				derecha = maximoRetardo(a.getHijoDerecho());
			}
			if (izquierda < derecha) {
				max = a.getDato() + derecha;
			} else  {
				max = a.getDato() + izquierda;
			}
		} 
		return max;
	}
	
	public int retardoReenvio() {
		int aux;
		aux = maximoRetardo(raiz);
		return aux;
	}
}