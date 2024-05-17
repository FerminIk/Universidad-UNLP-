package ar.edu.unlp.info.oo1.ejercicio4;

public class Cuerpo3D {
	private double altura;
	private Figura2D figura;
	
	public double getAltura() {
		return altura;
	}
	public void setAltura(double altura) {
		this.altura = altura;
	}
	
	public void setCaraBasal(Figura2D figura) {
		this.figura = figura;
	}
	
	public double getVolumen() {
		return this.figura.getArea() * this.altura;
	}
	
	public double getSuperficieExterior() {
		return (2*this.figura.getArea() + this.figura.getPerimetro() * this.altura);
	}
}
