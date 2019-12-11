function X = selectInBox(V, min_corner, max_corner)
    % select points inside a box
    X = [];

    for i = 1:size(V, 1)

        if (~(V(i, 1) < min_corner(1) || V(i, 1) > max_corner(1) || V(i, 2) < min_corner(2) || V(i, 2) > max_corner(2)))
            X = [X; V(i, :)];
        end

    end

end
