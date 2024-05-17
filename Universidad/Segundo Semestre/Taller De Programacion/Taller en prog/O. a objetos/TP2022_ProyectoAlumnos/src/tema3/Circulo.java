/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package tema3;

/**
 *
 * @author Mar
 */
public class Circulo {
    private double Radio;
    private String ColorInterno;
    private String ColorLinea;

    public Circulo(double Radio, String ColorInterno, String ColorLinea) {
        this.Radio = Radio;
        this.ColorInterno = ColorInterno;
        this.ColorLinea = ColorLinea;
    }

    public Circulo() {
    }

    public void setRadio(double Radio) {
        this.Radio = Radio;
    }

    public void setColorInterno(String ColorInterno) {
        this.ColorInterno = ColorInterno;
    }

    public void setColorLinea(String ColorLinea) {
        this.ColorLinea = ColorLinea;
    }

    public double getRadio() {
        return Radio;
    }

    public String getColorInterno() {
        return ColorInterno;
    }

    public String getColorLinea() {
        return ColorLinea;
    }
            
   public double calcularPerimetro() {
       
       return getRadio()*Math. PI*2;
       
   }
   
   public double calcularArea() {
       
       return Math.PI * Math.pow(this.Radio,2);
   }
}
