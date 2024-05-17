package ar.edu.unlp.info.oo1.ejercicio3;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

public class Presupuesto {
	private LocalDate fecha;
	private String cliente;
	private List<Item> items;
	
	public Presupuesto() {
		super();
		this.fecha = LocalDate.now();
		this.items =  new ArrayList<Item>();
	}
	public Presupuesto(String cliente) {
		super();
		this.fecha = LocalDate.now();
		this.cliente = cliente;
		this.items =  new ArrayList<Item>();
	}
	public LocalDate getFecha() {
		return fecha;
	}
	public void setFecha(LocalDate fecha) {
		this.fecha = fecha;
	}
	public String getCliente() {
		return cliente;
	}
	public void setCliente(String cliente) {
		this.cliente = cliente;
	}
	
	public List<Item> getItems() {
		return items;
	}
	public void setItems(List<Item> items) {
		this.items = items;
	}
	public void agregarItem(Item item) {
		this.items.add(item);
	}
	
	public double calcularTotal() {
		double aux = 0;
		Iterator<Item> itr =this.items.listIterator();
		while (itr.hasNext()) {
			aux = aux + itr.next().costo();
		}
		return aux;
	}
}
