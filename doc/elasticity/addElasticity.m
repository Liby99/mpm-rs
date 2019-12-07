function force = addElasticity(force, grid, xp, Fp, Vp0, mu, lambda)
% add elasticity to the force

Np = size(Fp,1);

for p = 1:Np
    
    thisFp = reshape(Fp(p,:,:),2,2);
    thisP = fixedCorotated( thisFp, mu, lambda );
    Vp0PFt = Vp0(p) * thisP * (thisFp');
    
    X = xp(p,:);
    X_index_space = X/grid.dx;    
    
    [w1, dw1, base_node1] = computeWeightsWithGradients1D( X_index_space(1) );
    [w2, dw2, base_node2] = computeWeightsWithGradients1D( X_index_space(2) );    
    
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
            foo = -Vp0PFt*grad_w;
            
            for d = 1:2
                force(node_i, node_j, d) = force(node_i, node_j, d) + foo(d);
            end
            
        end
    end

end


end

