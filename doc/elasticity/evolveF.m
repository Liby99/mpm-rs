function Fp = evolveF(dt, grid, vg, xp, Fp);
% evolve deformation gradient

Np = size(Fp,1);

for p = 1:Np
    
    thisFp = reshape(Fp(p,:,:),2,2);

    X = xp(p,:);
    X_index_space = X/grid.dx;    
    
    [w1, dw1, base_node1] = computeWeightsWithGradients1D( X_index_space(1) );
    [w2, dw2, base_node2] = computeWeightsWithGradients1D( X_index_space(2) );    
    
    % compute grad_vp
    grad_vp = zeros(2,2);    
    for i = 1:3
        wi = w1(i);
        dwidxi = dw1(i)/grid.dx;
        node_i = base_node1 + (i-1);
        for j = 1:3
            wj = w2(j);
            wij = wi * wj;
            dwijdxi = dwidxi * wj;
            dwijdxj = wi/grid.dx * dw2(j);
            node_j = base_node2 + (j-1);                    
            grad_w = [dwijdxi; dwijdxj];
            vij = [vg(node_i,node_j,1); vg(node_i,node_j,2)];
            grad_vp = grad_vp + vij*(grad_w');
        end
    end
    
    newFp = (eye(2) + dt*grad_vp) * thisFp;
    
    for i=1:2
        for j=1:2
            Fp(p,i,j) = newFp(i,j);
        end
    end

end


end

