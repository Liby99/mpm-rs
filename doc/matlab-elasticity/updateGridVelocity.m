function vg = updateGridVelocity(mg, vgn, force, active_nodes, dt, vg)
    % update velocity from force

    for k = 1:size(active_nodes, 1)
        i = active_nodes(k, 1);
        j = active_nodes(k, 2);

        for d = 1:2
            vg(i, j, d) = vgn(i, j, d) + dt * force(i, j, d) / mg(i, j);
        end

    end

end
