
package tema1;

//Paso 1: Importar la funcionalidad para lectura de datos
import PaqueteLectura.Lector;
public class Ej02Jugadores {

  
    public static void main(String[] args) {
        int DF = 15;
        double [] jugadores = new double [DF];
        int i, cant = 0;
        double total = 0, res;
        for (i=0;i<DF;i++){
            System.out.println("Jugador " + (i+1) + " ");
            jugadores[i] = Lector.leerDouble();
            System.out.println();
            total = jugadores[i]+total;
            cant++;
        }
        res =  (double) total/cant;
        System.out.println("El promedio de altura es: "+ res);
        cant = 0;
        for (i=0;i<DF;i++){
            if (jugadores[i]>=res)
                cant++;
        }
        System.out.println("la cantidad de jugadores mayores a la media es de: "+cant);
        
        //Paso 3: Crear el vector para 15 double 
        
        //Paso 4: Declarar indice y variables auxiliares a usar
                
        //Paso 6: Ingresar 15 numeros (altura), cargarlos en el vector, ir calculando la suma de alturas
        
        //Paso 7: Calcular el promedio de alturas, informarlo
        
        //Paso 8: Recorrer el vector calculando lo pedido (cant. alturas que están por encima del promedio)
     
        //Paso 9: Informar la cantidad.
    }
    
}
