package ar.edu.unlp.oo1.ejercicio1.impl;

import ar.edu.unlp.oo1.ejercicio1.WallPost;

/**
 * Completar esta clase de acuerdo a lo especificado en el cuadernillo
 *
 */
public class WallPostImpl implements WallPost {

	private String text;
	private int likes;
	private boolean marca;
	
	/*
	* Permite construir una instancia del WallpostImpl.
	* Luego de la invocación, debe tener como texto: “Undefined post”,
	* no debe estar marcado como destacado y la cantidad de “Me gusta” debe ser 0.
	*/

	
	public WallPostImpl() {
		this.text = "Undefined post";
		marca = false;
		likes = 0;
	}
	
	/*
	* Retorna el texto descriptivo de la publicación
	*/

	
	public String getText() {
		return text;
	}

	/*
	* Asigna el texto descriptivo de la publicación
	*/



	public void setText(String text) {
		this.text = text;
	}

	/*
	* Retorna la cantidad de “me gusta”
	*/


	public int getLikes() {
		return likes;
	}


	/*
	 * Incrementa la cantidad de likes en uno.
	*/

	public void like() {
		this.likes++;
	}
	
	/*
	 * Decrementa la cantidad de likes en uno. Si ya es 0, no hace nada.
	*/
	
	public void dislike() {
		if (this.likes != 0) {
			this.likes--;
		} 
	}


	public void setLikes(int likes) {
		this.likes = likes;
	}


	/*
	 * Retorna true si el post está marcado como destacado, false en caso contrario
	*/


	public boolean isFeatured() {
		return marca;
	}


	/*
	 * Cambia el post del estado destacado a no destacado y viceversa.
	*/


	public void toggleFeatured() {
		if (this.marca) {
			this.marca = false;
		} else {
			this.marca = true;
		}
	}



	/*
	 * Este mensaje se utiliza para que una instancia de Wallpost se muestre de forma adecuada
	 */
    @Override
    public String toString() {
        return "WallPost {" +
            "text: " + getText() +
            ", likes: '" + getLikes() + "'" +
            ", featured: '" + isFeatured() + "'" +
            "}";
    }



}
