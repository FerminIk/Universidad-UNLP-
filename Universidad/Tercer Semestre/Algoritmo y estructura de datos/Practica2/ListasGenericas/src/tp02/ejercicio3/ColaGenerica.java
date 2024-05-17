package tp02.ejercicio3;

import tp02.ejercicio2.NodoGenerico;

public class ColaGenerica<T> {
	private NodoGenerico<T> inicio;
	private NodoGenerico<T> ant;
	private NodoGenerico<T> fin;
	private int tamanio = 0;
	
	public void encolar(T elem) {
		NodoGenerico<T> aux = new NodoGenerico<T>();
		aux.setDato(elem);
		if (inicio == null) {
			inicio = aux;
			ant = aux;
			fin = aux;
		} else {
			fin.setSiguiente(aux);
			ant = fin;
			fin = aux;
		}
		tamanio++;
	}
	
	public T desencolar() {
		T elem = null;
		if (inicio != null) {
			elem = fin.getDato();
			fin = ant;
			ant = inicio;
			tamanio = tamanio -1;
			for (int i = 0; i<tamanio;i++) {
				ant = ant.getSiguiente();
			}
		}
		
		return elem;
	}
	
	public T tope() {
		T elem = null;
		if (inicio != null) {
			elem = fin.getDato();
		}
		return elem;
	}
	
	public boolean esVacia() {
		boolean aux;
		if (inicio == null) {
			aux = true;
		} else {
			aux = false;
		}
		return aux;
	}
}
