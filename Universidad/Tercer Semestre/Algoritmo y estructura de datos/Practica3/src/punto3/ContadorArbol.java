package punto3;

import ejercicio1.ArbolBinario;
import practica2.ListaEnlazadaGenerica;
import java.util.Scanner;

public class ContadorArbol<Integer> {
	private ArbolBinario<Integer> raiz;

	public ContadorArbol(ArbolBinario<Integer> raiz) {
		super();
		this.raiz = raiz;
	}

	public ContadorArbol() {
		super();
	}

	public ArbolBinario<Integer> getRaiz() {
		return raiz;
	}

	public void setRaiz(ArbolBinario<Integer> raiz) {
		this.raiz = raiz;
	}
	
	private boolean comprobar(Integer dato) {
		int aux = (int) dato;
		if (aux % 2 == 0) {
			return true;
		} else  {
			return false;
		}
		
	}
	
	private ListaEnlazadaGenerica<Integer> InOrden(ArbolBinario<Integer> a, ListaEnlazadaGenerica<Integer> aux) {
		if (!a.esVacio()) {
			Integer dato =a.getDato();
			if (a.tieneHijoIzquierdo()) {
				aux = InOrden(a.getHijoIzquierdo(),aux);
			}
			if (comprobar(dato)) {
				aux.agregarFinal(dato);
			}
			if (a.tieneHijoDerecho()) {
				aux = InOrden(a.getHijoDerecho(),aux);
			}
			
		}
		return aux;
	}
	
	private ListaEnlazadaGenerica<Integer> PreOrden(ArbolBinario<Integer> a, ListaEnlazadaGenerica<Integer> aux) {
		if (!a.esVacio()) {
			Integer dato =a.getDato();
			if (a.tieneHijoIzquierdo()) {
				aux = PreOrden(a.getHijoIzquierdo(),aux);
			}
			if (a.tieneHijoDerecho()) {
				aux = PreOrden(a.getHijoDerecho(),aux);
			}
			if (comprobar(dato)) {
				aux.agregarFinal(dato);
			}
		}
		return aux;
	}
	
	
	public ListaEnlazadaGenerica<Integer> numerosPares() {
		ListaEnlazadaGenerica<Integer> aux = new ListaEnlazadaGenerica<Integer>();
		System.out.println("1. se ejecuta el metodo en InOrden");
		System.out.println("2. se ejecuta el metodo en PreOrden");
		Scanner s = new Scanner(System.in);
		int num = s.nextInt();
		if (raiz.esVacio()) {
			return null;
		}else {
			while (true) {
				if (num == 1) {
					return InOrden(raiz,aux);
				} else if (num == 2) {
					return PreOrden(raiz,aux);
				}
				System.out.println("numero no valido, por favor introducelo nuevamente");
				num = s.nextInt();
			}
		}
	}
}
