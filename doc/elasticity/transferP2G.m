function [mg, vg, active_nodes] = transferP2G(xp, mp, vp, grid, mg, vg)
% P2G
% This assumes mg and vg is already resized and initialized with values (0)
Np = size(mp,1);

for p = 1:Np
    X = xp(p,:);
    X_index_space = X/grid.dx;
    [w1, base_node1] = computeWeights1D( X_index_space(1) );
    [w2, base_node2] = computeWeights1D( X_index_space(2) );

    for i = 1:3
        wi = w1(i);
        node_i = base_node1 + (i-1);
        for j = 1:3
            wij = wi * w2(j);
            node_j = base_node2 + (j-1); 
                        
            % splat mass
            mg(node_i, node_j) = mg(node_i, node_j) + mp(p) * wij;            
            % splat momentum
            for d = 1:2
                vg(node_i, node_j, d) = vg(node_i, node_j, d) + (wij * mp(p)) * vp(p,d);
            end
        end
    end

end

active_nodes = [];
for i = 1:size(mg,1)
    for j = 1:size(mg,2)
        if mg(i,j)~=0
            active_nodes = [active_nodes; [i,j]];
            vg(i, j, :) = vg(i, j, :) / mg(i,j);
        else
            vg(i, j, :) = 0;
        end
    end
end   

end

