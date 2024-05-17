package ar.edu.unlp.info.oo1.ejercicio4;

public class Circulo implements Figura2D{
	private double radio;

	public Circulo() {
		super();
		// TODO Auto-generated constructor stub
	}

	public double getMedida() {
		return radio;
	}

	public void setMedida(double radio) {
		this.radio = radio;
	}
	
	public double getDiametro() {
		return radio * 2;
	}

	public void setDiametro(double diametro) {
		if (diametro >= 0) {
			this.radio = diametro/2;
		} else {
			this.setMedida(0);
		}
	}
	
	public double getPerimetro() {
		return (Math.PI * this.getDiametro());
	}
	
	public double getArea() {
		return (Math.PI * (this.getMedida() * this.getMedida()));
	}
}
