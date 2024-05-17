package ejercicio1;

import practica2.ColaGenerica;

public class ArbolBinario<T> {
	private T dato;
	private ArbolBinario<T> hijoIzquierdo;   
	private ArbolBinario<T> hijoDerecho; 

	
	public ArbolBinario() {
		super();
	}

	public ArbolBinario(T dato) {
		this.dato = dato;
	}

	/*
	 * getters y setters
	 * 
	 */
	public T getDato() {
		return dato;
	}

	public void setDato(T dato) {
		this.dato = dato;
	}
	
	/**
	 * Preguntar antes de invocar si tieneHijoIzquierdo()
	 * @return
	 */
	public ArbolBinario<T> getHijoIzquierdo() {
		return this.hijoIzquierdo;
	}

	public ArbolBinario<T> getHijoDerecho() {
		return this.hijoDerecho;

	}

	public void agregarHijoIzquierdo(ArbolBinario<T> hijo) {
		this.hijoIzquierdo = hijo;
	}

	public void agregarHijoDerecho(ArbolBinario<T> hijo) {
		this.hijoDerecho = hijo;
	}

	public void eliminarHijoIzquierdo() {
		this.hijoIzquierdo = null;
	}

	public void eliminarHijoDerecho() {
		this.hijoDerecho = null;
	}

	public boolean esVacio() {
		return this.getDato() == null && !this.tieneHijoIzquierdo() && !this.tieneHijoDerecho();
	}

	public boolean esHoja() {
		return (!this.tieneHijoIzquierdo() && !this.tieneHijoDerecho());

	}

	@Override
	public String toString() {
		return this.getDato().toString();
	}

	public String imprimir(ArbolBinario<T> a) {
		String cadena = "";
		if (!a.esVacio()) {
			cadena = cadena + a.toString();
			if (a.tieneHijoIzquierdo()) {
				cadena = cadena + a.imprimir(a.getHijoIzquierdo());
			}
			if (a.tieneHijoDerecho()) {
				cadena = cadena + a.imprimir(a.getHijoDerecho());
			}
		}
		return cadena;
	}
	 
	public boolean tieneHijoIzquierdo() {
		return this.hijoIzquierdo!=null;
	}

	 
	public boolean tieneHijoDerecho() {
		return this.hijoDerecho!=null;
	}

	public int contarHojas(ArbolBinario<T> a) {
		int n = 0;
		if (!a.esVacio()) {
			n++;
			if (a.tieneHijoIzquierdo()) {
				n += contarHojas(a.getHijoIzquierdo());
			}
			if (a.tieneHijoDerecho()) {
				n += contarHojas(a.getHijoDerecho());
			}
		}
		return n;
	}
	

    public ArbolBinario<T> espejo(ArbolBinario<T> a) {
		if (!a.esVacio()) {
			if ((a.tieneHijoIzquierdo()) && (a.tieneHijoDerecho())) {
				ArbolBinario<T> aux = new ArbolBinario<T>();
				aux = a.getHijoIzquierdo();
				a.agregarHijoIzquierdo(a.getHijoDerecho());
				a.agregarHijoDerecho(aux);
				a.espejo(a.getHijoIzquierdo());
				a.espejo(a.getHijoDerecho());
			} else {
				if (a.tieneHijoIzquierdo()) {
					a.agregarHijoDerecho(a.getHijoIzquierdo());
					a.agregarHijoIzquierdo(null);
					a.espejo(a.getHijoDerecho());
				} else {
					if (a.tieneHijoDerecho()) {
						a.agregarHijoIzquierdo(a.getHijoDerecho());
						a.agregarHijoDerecho(null);
						a.espejo(a.getHijoIzquierdo());
					}
				}
			}
		}
		return a;
	}


	public  void entreNiveles(int n, int m){
		ArbolBinario<T> aux = null;
		ColaGenerica<ArbolBinario<T>> cola = new ColaGenerica<ArbolBinario<T>>();
		cola.insertar(this);
		cola.insertar(null);
		int act = 0;
		while (!cola.pilaVacia()) {
			aux = cola.descolar();
			if (aux != null) {
				if ((act >= n) && (act <=m)) {
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
	}

	

}
