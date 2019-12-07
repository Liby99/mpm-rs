function [ result ] = computeParticleMomentum( mp, vp )
% compute total particle momentum

result = zeros(2,1);
Np = size(mp,1);

for p =  1:Np
    
    for d = 1:2
        result(d) = result(d) + mp(p)*vp(p,d);
    end
    
end

end

