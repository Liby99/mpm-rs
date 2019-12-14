function vg = setBoundaryVelocities(thickness, vg)
    % set domain boundary velocities

    Nx = size(vg, 1);
    Ny = size(vg, 2);

    for i = 1:thickness

        for j = 1:Ny
            vg(i, j, :) = 0;
        end

    end

    for i = Nx - thickness + 1:Nx

        for j = 1:Ny
            vg(i, j, :) = 0;
        end

    end

    for i = 1:Nx

        for j = 1:3
            vg(i, j, :) = 0;
        end

    end

    for i = 1:Nx

        for j = Ny - thickness + 1:Ny
            vg(i, j, :) = 0;
        end

    end

end
