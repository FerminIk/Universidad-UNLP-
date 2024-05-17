package ar.edu.unlp.info.oo1.ejercicio2;

public class Producto {
	private double Peso;
	private double precioPorKilo;
	private String descripcion;
	
	public Producto(double peso, double precioPorKilo, String descripcion) {
		super();
		Peso = peso;
		this.precioPorKilo = precioPorKilo;
		this.descripcion = descripcion;
	}
	
	

	public Producto() {
		super();
		// TODO Auto-generated constructor stub
	}



	public double getPeso() {
		return Peso;
	}

	public void setPeso(double peso) {
		Peso = peso;
	}

	public double getPrecioPorKilo() {
		return precioPorKilo;
	}

	public void setPrecioPorKilo(double precioPorKilo) {
		this.precioPorKilo = precioPorKilo;
	}

	public String getDescripcion() {
		return descripcion;
	}

	public void setDescripcion(String descripcion) {
		this.descripcion = descripcion;
	}
	
	public double getPrecio() {
		double aux = 0;
		if ((this.getPeso() >= 0) && (this.getPrecioPorKilo()  >= 0)) {
			aux = this.getPeso() * this.getPrecioPorKilo();
		}
		return aux;
	}
	
}
