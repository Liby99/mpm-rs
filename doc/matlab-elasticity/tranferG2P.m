function [xp, vp] = tranferG2P(dt, grid, vgn, vg, flip, xp, vp)
    % G2P
    Np = size(xp, 1);

    for p = 1:Np
        X = xp(p, :);
        X_index_space = X / grid.dx;

        [w1, base_node1] = computeWeights1D(X_index_space(1));
        [w2, base_node2] = computeWeights1D(X_index_space(2));

        vpic = zeros(2, 1);
        vflip = [vp(p, 1); vp(p, 2)];

        for i = 1:3
            wi = w1(i);
            node_i = base_node1 + (i - 1);

            for j = 1:3
                wij = wi * w2(j);
                node_j = base_node2 + (j - 1);

                for d = 1:2
                    vpic(d) = vpic(d) + wij * vg(node_i, node_j, d);
                    vflip(d) = vflip(d) + wij * (vg(node_i, node_j, d) - vgn(node_i, node_j, d));
                end

            end

        end

        for d = 1:2
            vp(p, d) = (1 - flip) * vpic(d) + flip * vflip(d);
            xp(p, d) = xp(p, d) + dt * vpic(d);
        end

    end

end
