package ar.edu.unlp.info.oo1.ejercicio4;

public class Cuadrado implements Figura2D {
	private double lado;

	public Cuadrado() {
		super();
		// TODO Auto-generated constructor stub
	}

	public double getMedida() {
		return lado;
	}

	public void setMedida(double lado) {
		this.lado = lado;
	}
	
	public double getPerimetro() {
		return lado * 4;
	}

	public double getArea() {
		return lado * lado;
	}


}
