#[macro_export]
macro_rules! s0 {
    ($x:expr) => {
        $x.rotate_right(7) ^ $x.rotate_right(18) ^ ($x >> 3)
    };
}

#[macro_export]
macro_rules! s1 {
    ($x:expr) => {
        $x.rotate_right(17) ^ $x.rotate_right(19) ^ ($x >> 10)
    };
}

#[macro_export]
macro_rules! S0 {
    ($x:expr) => {
        $x.rotate_right(2) ^ $x.rotate_right(13) ^ $x.rotate_right(22)
    };
}

#[macro_export]
macro_rules! S1 {
    ($x:expr) => {
        $x.rotate_right(6) ^ $x.rotate_right(11) ^ $x.rotate_right(25)
    };
}

#[macro_export]
macro_rules! CH {
    ($e:expr, $f:expr, $g:expr) => {
        ($e & $f) ^ ((!$e) & $g)
    };
}

#[macro_export]
macro_rules! MAJ {
    ($a:expr, $b:expr, $c:expr) => {
        ($a & $b) ^ ($a & $c) ^ ($b & $c)
    };
}

#[macro_export]
macro_rules! expand_w_line {
    ($w:ident, $i:expr) => {
        $w[$i] = $w[$i - 16]
            .wrapping_add(s0!($w[$i - 15]))
            .wrapping_add($w[$i - 7])
            .wrapping_add(s1!($w[$i - 2]));
    };
}

#[macro_export]
macro_rules! expand_w {
    ($w:ident) => {
        expand_w_line!($w, 16);
        expand_w_line!($w, 17);
        expand_w_line!($w, 18);
        expand_w_line!($w, 19);
        expand_w_line!($w, 20);
        expand_w_line!($w, 21);
        expand_w_line!($w, 22);
        expand_w_line!($w, 23);
        expand_w_line!($w, 24);
        expand_w_line!($w, 25);
        expand_w_line!($w, 26);
        expand_w_line!($w, 27);
        expand_w_line!($w, 28);
        expand_w_line!($w, 29);
        expand_w_line!($w, 30);
        expand_w_line!($w, 31);

        expand_w_line!($w, 32);
        expand_w_line!($w, 33);
        expand_w_line!($w, 34);
        expand_w_line!($w, 35);
        expand_w_line!($w, 36);
        expand_w_line!($w, 37);
        expand_w_line!($w, 38);
        expand_w_line!($w, 39);
        expand_w_line!($w, 40);
        expand_w_line!($w, 41);
        expand_w_line!($w, 42);
        expand_w_line!($w, 43);
        expand_w_line!($w, 44);
        expand_w_line!($w, 45);
        expand_w_line!($w, 46);
        expand_w_line!($w, 47);

        expand_w_line!($w, 48);
        expand_w_line!($w, 49);
        expand_w_line!($w, 50);
        expand_w_line!($w, 51);
        expand_w_line!($w, 52);
        expand_w_line!($w, 53);
        expand_w_line!($w, 54);
        expand_w_line!($w, 55);
        expand_w_line!($w, 56);
        expand_w_line!($w, 57);
        expand_w_line!($w, 58);
        expand_w_line!($w, 59);
        expand_w_line!($w, 60);
        expand_w_line!($w, 61);
        expand_w_line!($w, 62);
        expand_w_line!($w, 63);
    };
}

#[macro_export]
macro_rules! ROUND {
    ($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$g:ident,$h:ident, $w:expr,$k:expr) => {{
        let t1 = $h
            .wrapping_add(S1!($e))
            .wrapping_add(CH!($e, $f, $g))
            .wrapping_add($k)
            .wrapping_add($w);

        let t2 = S0!($a).wrapping_add(MAJ!($a, $b, $c));

        $h = $g;
        $g = $f;
        $f = $e;
        $e = $d.wrapping_add(t1);
        $d = $c;
        $c = $b;
        $b = $a;
        $a = t1.wrapping_add(t2);
    }};
}

#[macro_export]
macro_rules! all_rounds {
    ($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$g:ident,$h:ident,$w:ident,$k:ident) => {
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[0], $k[0]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[1], $k[1]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[2], $k[2]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[3], $k[3]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[4], $k[4]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[5], $k[5]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[6], $k[6]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[7], $k[7]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[8], $k[8]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[9], $k[9]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[10], $k[10]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[11], $k[11]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[12], $k[12]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[13], $k[13]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[14], $k[14]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[15], $k[15]);

        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[16], $k[16]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[17], $k[17]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[18], $k[18]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[19], $k[19]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[20], $k[20]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[21], $k[21]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[22], $k[22]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[23], $k[23]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[24], $k[24]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[25], $k[25]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[26], $k[26]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[27], $k[27]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[28], $k[28]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[29], $k[29]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[30], $k[30]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[31], $k[31]);

        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[32], $k[32]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[33], $k[33]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[34], $k[34]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[35], $k[35]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[36], $k[36]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[37], $k[37]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[38], $k[38]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[39], $k[39]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[40], $k[40]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[41], $k[41]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[42], $k[42]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[43], $k[43]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[44], $k[44]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[45], $k[45]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[46], $k[46]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[47], $k[47]);

        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[48], $k[48]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[49], $k[49]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[50], $k[50]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[51], $k[51]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[52], $k[52]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[53], $k[53]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[54], $k[54]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[55], $k[55]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[56], $k[56]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[57], $k[57]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[58], $k[58]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[59], $k[59]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[60], $k[60]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[61], $k[61]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[62], $k[62]);
        ROUND!($a, $b, $c, $d, $e, $f, $g, $h, $w[63], $k[63]);
    };
}
