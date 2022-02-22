use crate::shape::point::Point;

/// https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
pub(crate) fn xiaolin_wu_line(x0: f32, y0: f32, x1: f32, y1: f32) -> Vec<Point> {
    let mut pixels = Vec::new();

    //  boolean steep := abs(y1 - y0) > abs(x1 - x0)
    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    //  if steep then
    //      swap(x0, y0)
    //      swap(x1, y1)
    //  end if
    let (x0, y0, x1, y1) = if steep {
        (y0, x0, y1, x1)
    } else {
        (x0, y0, x1, y1)
    };

    //  if x0 > x1 then
    //      swap(x0, x1)
    //      swap(y0, y1)
    //  end if
    let (x0, y0, x1, y1) = if x0 > x1 {
        (x1, y1, x0, y0)
    } else {
        (x0, y0, x1, y1)
    };

    //  dx := x1 - x0
    //  dy := y1 - y0
    let dx = x1 - x0;
    let dy = y1 - y0;

    //  if dx == 0.0 then
    //      gradient := 1.0
    //  else
    //      gradient := dy / dx
    //  end if
    let gradient = if dx == 0. { 1.0 } else { dy / dx };

    //  handle first endpoint
    //  xend := round(x0)
    //  yend := y0 + gradient * (xend - x0)
    //  xgap := rfpart(x0 + 0.5)
    //  xpxl1 := xend // this will be used in the main loop
    //  ypxl1 := ipart(yend)
    let xend = round(x0);
    let yend = y0 + gradient * (xend - x0);
    let xgap = rfpart(x0 + 0.5);
    let xpxl1 = xend;
    let ypxl1 = ipart(yend);

    //  if steep then
    //      plot(ypxl1,   xpxl1, rfpart(yend) * xgap)
    //      plot(ypxl1+1, xpxl1,  fpart(yend) * xgap)
    //  else
    //      plot(xpxl1, ypxl1  , rfpart(yend) * xgap)
    //      plot(xpxl1, ypxl1+1,  fpart(yend) * xgap)
    //  end if
    pixels.push(plot(ypxl1, xpxl1, rfpart(yend) * xgap));
    if steep {
        pixels.push(plot(ypxl1 + 1., xpxl1, fpart(yend) * xgap));
    } else {
        pixels.push(plot(xpxl1, ypxl1 + 1., fpart(yend) * xgap));
    }

    //  intery := yend + gradient
    let mut intery = yend + gradient;

    //  handle second endpoint
    //  xend := round(x1)
    //  yend := y1 + gradient * (xend - x1)
    //  xgap := fpart(x1 + 0.5)
    //  xpxl2 := xend //this will be used in the main loop
    //  ypxl2 := ipart(yend)
    let xend = round(x1);
    let yend = y1 + gradient * (xend - x1);
    let xgap = fpart(x1 + 0.5);
    let xpxl2 = xend;
    let ypxl2 = ipart(yend);

    //  if steep then
    //    plot(ypxl2  , xpxl2, rfpart(yend) * xgap)
    //    plot(ypxl2+1, xpxl2,  fpart(yend) * xgap)
    //  else
    //    plot(xpxl2, ypxl2,  rfpart(yend) * xgap)
    //    plot(xpxl2, ypxl2+1, fpart(yend) * xgap)
    //  end if
    pixels.push(plot(ypxl2, xpxl2, rfpart(yend) * xgap));
    if steep {
        pixels.push(plot(ypxl2 + 1., xpxl2, fpart(yend) * xgap));
    } else {
        pixels.push(plot(xpxl2, ypxl2 + 1., fpart(yend) * xgap));
    }

    // main loop
    //  if steep then
    //    for x from xpxl1 + 1 to xpxl2 - 1 do
    //      plot(ipart(intery)  , x, rfpart(intery))
    //      plot(ipart(intery)+1, x,  fpart(intery))
    //      intery := intery + gradient
    //    end
    //  else
    //     for x from xpxl1 + 1 to xpxl2 - 1 do
    //       plot(x, ipart(intery),  rfpart(intery))
    //       plot(x, ipart(intery)+1, fpart(intery))
    //       intery := intery + gradient
    //     end
    //  end if
    let mut x = xpxl1 + 1.;
    while x < xpxl2 - 1. {
        if steep {
            pixels.push(plot(ipart(intery), x, rfpart(intery)));
            pixels.push(plot(ipart(intery) + 1., x, fpart(intery)));
        } else {
            pixels.push(plot(x, ipart(intery), rfpart(intery)));
            pixels.push(plot(x, ipart(intery) + 1., fpart(intery)));
        }
        x += 1.;
        intery += gradient
    }

    pixels
}

fn plot(x: f32, y: f32, brightness: f32) -> Point {
    let color = [255, 255, 255, (brightness * 255.) as u8];
    let point = Point::new(x as i32, y as i32, color);
    point
}

fn ipart(x: f32) -> f32 {
    x.floor()
}

fn round(x: f32) -> f32 {
    ipart(x + 0.5)
}

fn fpart(x: f32) -> f32 {
    x - x.floor()
}

fn rfpart(x: f32) -> f32 {
    1. - fpart(x)
}
