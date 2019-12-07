function force = addGravity(force, mg, active_nodes, gravity)
% add gravity to the force

for k = 1:size(active_nodes,1)
    i = active_nodes(k,1);
    j = active_nodes(k,2);
    for d = 1:2
        force(i,j,d) = force(i,j,d) + mg(i,j)*gravity(d);
    end
end


end

