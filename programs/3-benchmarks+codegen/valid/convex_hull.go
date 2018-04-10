package main;

type point struct {
    x,y int;
}
type chull struct {
    points []point;
    size int;
}

var window = 100000;
var no_points = 50000;
var point_set []point;

func ccw(a point, b point, c point) bool {
	var v int = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);
	if v == 0 {
		return false;
	}
	if v < 0 {
		return true
	}
	return false
}

func convex_hull(set []point) chull {
    var ch chull;

    var left,first,cur,prev  = window,0,0,0
    for i := 0; i<no_points; i++ {
        if left > set[i].x {
            left = set[i].x
            cur = i;

          first = i
        }
    }

    for  {
        ch.points = append(ch.points, set[cur])
        ch.size++
        println(ch.size);
        prev = cur;
        cur = (prev+1)%no_points
        for j := 0; j<no_points; j++ {
            if ccw(set[prev], set[j], set[cur]) {
                cur = j;
            }
        }
        if cur == first {
            break
        }
    }
    return ch;
}

func init() {
    for i := 0; i<no_points; i++ {
        var new_point point;
        new_point.x = ((i*1028+i*5%7)%(19993))*((i*360+i*7%5)%13) % window;
        new_point.y = ((i*344+i*5%7)%(19993))*((i*45*7*2*9 + i*7%5)%13) % window;
        point_set = append(point_set, new_point);
    }
}

func main() {
    for j :=0 ; j<50; j++ {
    println("============================================================");
    println(j)
    println("Convex hull of a set of points.");
    println("============================================================");
    println("# of points:", no_points);
    println("Points are");
    for i := 0; i<no_points; i++ {
        print("(", point_set[i].x, ", ", point_set[i].y, "), ");
    }
    println("")
    println("============================================================");
    println("Convex Hull is");
    var ch = convex_hull(point_set);
    for i := 0; i<ch.size; i++ {
        print("(", ch.points[i].x, ", ", ch.points[i].y, "), ");
    }
    println("")
}
}
