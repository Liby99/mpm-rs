function [ result ] = computeGridMomentum( mg, vg )
% compute total grid momentum

result = zeros(2,1);

for i =  1:size(mg,1)
    for j = 1:size(mg,2)
        for d = 1:2
            result(d) = result(d) + mg(i,j)*vg(i,j,d);
        end
    end
end

end

