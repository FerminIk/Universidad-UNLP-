import ejercicio1.ArbolBinario;
import punto3.ContadorArbol;
import practica2.ListaEnlazadaGenerica;
import ejercicio1.ArbolBinario;
import punto4.RedBinariaLlena;
import punto5.ProfundidadDeArbolBinario;

public class main {

	public static void main(String[] args) {
		ArbolBinario<Integer> a1 = new ArbolBinario<Integer>(1);
		ArbolBinario<Integer> a2 = new ArbolBinario<Integer>(2);
		ArbolBinario<Integer> a3 = new ArbolBinario<Integer>(3);
		ArbolBinario<Integer> a4 = new ArbolBinario<Integer>(4);
		ArbolBinario<Integer> a5 = new ArbolBinario<Integer>(5);
		ArbolBinario<Integer> a6 = new ArbolBinario<Integer>(6);
		ArbolBinario<Integer> a7 = new ArbolBinario<Integer>(7);
		a2.agregarHijoDerecho(a4);
		a2.agregarHijoIzquierdo(a5);
		a3.agregarHijoDerecho(a6);
		a3.agregarHijoIzquierdo(a7);
		a1.agregarHijoIzquierdo(a2);
		a1.agregarHijoDerecho(a3);
		ProfundidadDeArbolBinario arbol = new ProfundidadDeArbolBinario(a1);
		System.out.println(arbol.sumaElementosProfundidad(2));
	}

}
