package ar.edu.unlp.info.oo1.ejercicio2;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;



public class Balanza {
	private Integer cantidadDeProductos;
	private List<Producto> listaProductos;
	
	public Balanza() {
		this.listaProductos =  new ArrayList<Producto>();
		this.ponerEnCero();
	}

	public Balanza(Integer cantidadDeProductos) {
		super();
		this.cantidadDeProductos = cantidadDeProductos;
		this.listaProductos =  new ArrayList<Producto>();
	}
	
	public Integer getCantidadDeProductos() {
		return cantidadDeProductos;
	}

	public Double getPrecioTotal() {
		double aux = 0.0;
		if (!this.listaProductos.isEmpty()) {
			Iterator<Producto> itr =this.listaProductos.listIterator();
			while (itr.hasNext()) {
				aux = aux + itr.next().getPrecio();
			}
		}
		return aux;
	}

	public Double getPesoTotal() {
		double aux = 0.0;
		if (!this.listaProductos.isEmpty()) {
			Iterator<Producto> itr =this.listaProductos.listIterator();
			while (itr.hasNext()) {
				aux = aux + itr.next().getPeso();
			}
		}
		return aux;
	}
	
	public List<Producto> getProductos() {
		return listaProductos;
	}
	
	private void agregarProductoLista(Producto producto) {
		this.listaProductos.add(producto);
	}
	
	public void ponerEnCero() {
		this.listaProductos.clear();
		this.cantidadDeProductos = 0;
	}
	
	public void agregarProducto(Producto producto) {
		this.agregarProductoLista(producto);
		this.cantidadDeProductos++;
	}
	
	public Ticket emitirTicket() {
		Ticket aux = new Ticket(this.getCantidadDeProductos(),this.getPesoTotal(),this.getPrecioTotal());
		return aux;
	}

}
