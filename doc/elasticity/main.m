% mpm 2d
clc; clear; close all;
rng(1);

if ~exist('tmp', 'dir')
    mkdir('tmp')
end

delete('tmp/*.png');

% simulation parameters
dt = 1e-3;
gravity = [0, -9.8];

% grid parameters
% (Don't change min_corner. It is assumed in weight computations)
grid = struct('min_corner', [0; 0], 'max_corner', [1; 1], 'dx', 0.02);
res = (grid.max_corner - grid.min_corner) / grid.dx + 1;

% sample particles
samples = poissonDisc([512, 512], 1);

for i = 1:size(samples, 1)
    samples(i, :) = samples(i, :) / 512.;
end

xp = selectInBox(samples, [0.3, 0.1], [0.5, 0.3]);

% set up particle attributes
E = 1e4;
nu = 0.3;
mu = E / (2 * (1 + nu));
lambda = E * nu / ((1 + nu) * (1 - 2 * nu));
rho = 1000;
Np = size(xp, 1);
Vp0 = zeros(Np, 1) + grid.dx^2/4;
mp = Vp0 * rho;
vp = zeros(Np, 2);
Fp = zeros(Np, 2, 2);

for p = 1:Np
    Fp(p, :, :) = eye(2); % use reshape(Fp(p,:,:),2,2) to get the 2x2 Fp
end

% plot
vis = figure(1);
scatter(xp(:, 1), xp(:, 2), 5.);
axis([grid.min_corner(1) grid.max_corner(1) grid.min_corner(2) grid.max_corner(2)]);
axis equal
plot_grid_x = linspace(grid.min_corner(1), grid.max_corner(1), res(1));
plot_grid_y = linspace(grid.min_corner(2), grid.max_corner(2), res(2));

for k = 1:length(plot_grid_y)
    line([plot_grid_x(1) plot_grid_x(end)], [plot_grid_y(k) plot_grid_y(k)])
end

for k = 1:length(plot_grid_x)
    line([plot_grid_x(k) plot_grid_x(k)], [plot_grid_y(1) plot_grid_y(end)])
end

axis square
drawnow
frame = 0;
saveas(vis, strcat('./tmp/frame', num2str(frame, '%03d'), '.png'));

for step = 1:100000
    fprintf('==================== Step %d ================= \n', step);

    % init zero grid data
    mg = zeros(res(1), res(2));
    vgn = zeros(res(1), res(2), 2);
    vg = zeros(res(1), res(2), 2);
    force = zeros(res(1), res(2), 2);

    % P2G
    Lp = computeParticleMomentum(mp, vp);
    fprintf('part momentum before p2g: %f, %f \n', Lp(1), Lp(2));

    [mg, vgn, active_nodes] = transferP2G(xp, mp, vp, grid, mg, vgn);

    Lg = computeGridMomentum(mg, vgn);
    fprintf('grid momentum after  p2g: %f, %f \n', Lg(1), Lg(2));

    % compute force
    force = addGravity(force, mg, active_nodes, gravity);
    force = addElasticity(force, grid, xp, Fp, Vp0, mu, lambda);

    % update velocity
    vg = updateGridVelocity(mg, vgn, force, active_nodes, dt, vg);

    % boundary conditions
    vg = setBoundaryVelocities(3, vg);

    % G2P (including particle advection)
    Lg = computeGridMomentum(mg, vg);
    fprintf('grid momentum before g2p: %f, %f \n', Lg(1), Lg(2));

    Fp = evolveF(dt, grid, vg, xp, Fp);
    [xp, vp] = tranferG2P(dt, grid, vgn, vg, 0.95, xp, vp);

    Lp = computeParticleMomentum(mp, vp);
    fprintf('part momentum after  g2p: %f, %f \n', Lp(1), Lp(2));

    if mod(step, 20) == 0
        frame = frame + 1;

        % plot
        scatter(xp(:, 1), xp(:, 2), 5.);
        axis([grid.min_corner(1) grid.max_corner(1) grid.min_corner(2) grid.max_corner(2)]);
        axis equal
        plot_grid_x = linspace(grid.min_corner(1), grid.max_corner(1), res(1));
        plot_grid_y = linspace(grid.min_corner(2), grid.max_corner(2), res(2));

        for k = 1:length(plot_grid_y)
            line([plot_grid_x(1) plot_grid_x(end)], [plot_grid_y(k) plot_grid_y(k)])
        end

        for k = 1:length(plot_grid_x)
            line([plot_grid_x(k) plot_grid_x(k)], [plot_grid_y(1) plot_grid_y(end)])
        end

        axis square

        drawnow
        saveas(vis, strcat('./tmp/frame', num2str(frame, '%03d'), '.png'));

        pause(0.001);
    end

end
