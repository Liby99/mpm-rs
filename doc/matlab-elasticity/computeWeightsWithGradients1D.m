function [w, dw, base_node] = computeWeightsWithGradients1D(x)
    % compute 1D quadratic B spline weights
    % x is assumed to be scaled in the index space (i.e., it is in a dx=1 grid)
    % w is 1x3 (row vector)

    base_node = floor(x - 0.5) + 1;
    w = zeros(1, 3);
    dw = zeros(1, 3);

    d0 = x - base_node + 1;
    z = 1.5 - d0;
    z2 = z * z;
    w(1) = 0.5 * z2;

    d1 = d0 - 1;
    w(2) = 0.75 - d1 * d1;

    d2 = 1 - d1;
    zz = 1.5 - d2;
    zz2 = zz * zz;
    w(3) = 0.5 * zz2;

    dw(1) = -z;
    dw(2) = -2 * d1;
    dw(3) = zz;

end
