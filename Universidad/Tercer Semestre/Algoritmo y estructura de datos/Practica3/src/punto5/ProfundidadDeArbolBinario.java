package punto5;

import ejercicio1.ArbolBinario;
import practica2.ColaGenerica;

public class ProfundidadDeArbolBinario {
	private ArbolBinario<Integer> raiz;
	
	
	public ArbolBinario<Integer> getRaiz() {
		return raiz;
	}



	public void setRaiz(ArbolBinario<Integer> raiz) {
		this.raiz = raiz;
	}



	public ProfundidadDeArbolBinario(ArbolBinario<Integer> raiz) {
		super();
		this.raiz = raiz;
	}



	public int sumaElementosProfundidad(int p){
		ArbolBinario<Integer> aux = null;
		ColaGenerica<ArbolBinario<Integer>> cola = new ColaGenerica<ArbolBinario<Integer>>();
		cola.insertar(raiz);
		cola.insertar(null);
		int act = 0;
		int total = 0;
		while (!cola.pilaVacia()) {
			aux = cola.descolar();
			if (aux != null) {
				if ((act <= p)) {
					total = total + aux.getDato();
					System.out.println(aux.toString());
				}
				if (aux.tieneHijoIzquierdo()) {
					cola.insertar(aux.getHijoIzquierdo());
				}
				if (aux.tieneHijoDerecho()) {
					cola.insertar(aux.getHijoDerecho());
				}
			} else if (!cola.pilaVacia()) {
				cola.insertar(null);
				act++;
			}
		}
		return total;
	}
	
}
