// Aula 2  - Tipos Primitivos em RUST



/*
      ===================== TIPOS ==========================


      Tipo1  - Escalares:
                            Representam um único valor contido dentro de uma escala conhecida.

                            Permitem a comparação direta entre valores.
        {
            Inteiro = (integer) ex: '5'
            Flutuante = (floating-point) ex: '45.1'
            Booleano = (boolean) ex: 'true', 'false'
            caractere = (char) ex: `'a''
        }

        # Inteiros:

        [ bits ]   [ signed ]  [ unsigned ]
        [ -------------------------------]
        [   8   ]   [  I8  ]    [  U8    ]
        [   16  ]   [ I16  ]    [  U16   ]
        [   32  ]   [ I32  ]    [  U32   ]
        [   64  ]   [ I64  ]    [  U64   ]
        [   128 ]   [ I128 ]    [  U128  ]
        [  arch ]   [isize ]    [  usize ]
        [ -------------------------------] 

        # Signed:
                range:   -(2^n-1) até 2^-1 -1
                i8:   -128 até 127  [-(2^7) até 2^7 -1]

        # Unsigned:
                range: 0 até 2^n -1
                u8: 0 até 255 [0 até 2^8 -1]


      Tipo2 - Compostos:

                            Servem para agregar multiplos valores.
        
        {
            Tupla ex: (5, true, 32.1, 'a')
            Matriz ex: `[1, 2, 3, 4, 5]´ 
        }


*/


fn main(){
    /*
    let x = 5;
    let y =  x * 200;
    o default é i32;
    */

    let x = 5;
    let y = 199_456_9898;
 


}