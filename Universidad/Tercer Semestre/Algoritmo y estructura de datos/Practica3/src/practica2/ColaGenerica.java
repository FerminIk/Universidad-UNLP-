package practica2;

public class ColaGenerica<T> {
	private ListaEnlazadaGenerica<T> tope = null;
	
	public ColaGenerica() {
		tope = new ListaEnlazadaGenerica<T>();
	}
	
	public boolean pilaVacia() {
		return tope.esVacia();
	}
	
	public void insertar(T elem) {
		tope.agregarFinal(elem);
	}
	
	public T topePila() {
		return tope.elemento(1);
	}
	
	public void reiniciarPila() {
		tope = null;
	}
	
	public T descolar() {
		T elem = tope.elemento(1);
		tope.eliminarEn(1);
		return elem;
	}
}